mod args;
mod configure;

use std::ffi::OsString;
use std::fs;
use std::fs::DirBuilder;
use std::io::ErrorKind;
use std::os::unix::fs::{DirBuilderExt, MetadataExt};
use std::path::Path;
use std::process::ExitCode;

use anyhow::{Context, Result};
use dialoguer::Confirm;
use tracing::{error, info, warn};
use tracing_subscriber::filter::EnvFilter;
use walkdir::WalkDir;

#[derive(Default)]
struct Stats {
    linked: u64,
    skipped: u64,
    overwritten: u64,
}

fn ensure_parent_dir(parent: &Path, mode: u32) -> Result<()> {
    if parent.exists() {
        return Ok(());
    }

    DirBuilder::new()
        .recursive(true)
        .mode(mode)
        .create(parent)
        .with_context(|| format!("creating directory {}", parent.display()))?;

    Ok(())
}

fn get_choices(settings: &configure::Settings) -> Result<Vec<OsString>> {
    if !settings.choices.is_empty() {
        return Ok(settings.choices.clone());
    }

    let mut choices = Vec::new();

    for entry in fs::read_dir(&settings.from)
        .with_context(|| format!("reading directory {}", settings.from.display()))?
    {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            choices.push(entry.file_name());
        }
    }

    if choices.is_empty() {
        anyhow::bail!("no choices specified");
    }

    Ok(choices)
}

fn is_already_linked(src: &Path, dest: &Path) -> Result<bool> {
    if let Ok(dest_meta) = fs::metadata(dest) {
        let src_meta = fs::metadata(src)
            .with_context(|| format!("reading metadata for source {}", src.display()))?;

        return Ok(dest_meta.ino() == src_meta.ino() && dest_meta.dev() == src_meta.dev());
    }

    Ok(false)
}

fn create_hard_link(
    src: &Path,
    dest: &Path,
    settings: &configure::Settings,
    stats: &mut Stats,
) -> Result<()> {
    if is_already_linked(src, dest)? {
        info!("{} already links to {}", dest.display(), src.display());
        stats.skipped += 1;
        return Ok(());
    }

    let parent = dest
        .parent()
        .with_context(|| format!("link {} has no parent directory", dest.display()))?;

    ensure_parent_dir(parent, settings.directory_mode)
        .with_context(|| format!("failed to create parent {}", parent.display()))?;

    loop {
        match fs::hard_link(src, dest) {
            Ok(_) => {
                info!("linked {} to {}", dest.display(), src.display());
                stats.linked += 1;
                break;
            }
            Err(err) if err.kind() == ErrorKind::AlreadyExists => {
                let should_overwrite = if settings.force {
                    true
                } else if settings.interactive {
                    let prompt = format!("Overwrite {}?", dest.display());
                    Confirm::new().with_prompt(prompt).interact()?
                } else {
                    false
                };

                if !should_overwrite {
                    warn!(
                        "skipping existing {} (use --force to overwrite or --interactive for manual intervention)",
                        dest.display()
                    );

                    stats.skipped += 1;

                    break;
                }

                info!("removing existing {}", dest.display());

                fs::remove_file(dest)
                    .with_context(|| format!("removing existing {}", dest.display()))?;

                stats.overwritten += 1;
            }
            Err(err) => {
                return Err(err).with_context(|| {
                    format!("creating hard link {} to {}", dest.display(), src.display())
                });
            }
        }
    }

    Ok(())
}

fn process_choice(
    choice: &OsString,
    settings: &configure::Settings,
    stats: &mut Stats,
) -> Result<()> {
    let prefix = settings.from.join(choice);

    if !prefix.is_dir() {
        warn!("skipping {}, not a directory", prefix.display());
        return Ok(());
    }

    for entry in WalkDir::new(&prefix)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let rel = path
            .strip_prefix(&prefix)
            .with_context(|| format!("computing relative path for {}", path.display()))?;

        let dest = settings.to.join(rel);
        create_hard_link(path, &dest, settings, stats)?;
    }

    Ok(())
}

fn run(settings: &configure::Settings) -> Result<()> {
    if settings.verbose {
        info!(
            "weaving from {} to {}",
            settings.from.display(),
            settings.to.display()
        );
    }

    let choices = get_choices(&settings)?;

    info!("choices are {:?}", choices);

    let mut stats = Stats::default();

    for choice in &choices {
        process_choice(choice, &settings, &mut stats)?;
    }

    info!(
        "{} linked, {} overwritten, {} skipped",
        stats.linked, stats.overwritten, stats.skipped
    );

    Ok(())
}

fn main() -> ExitCode {
    let settings = match configure::configure() {
        Ok(p) => p,
        Err(err) => {
            error!("failed to configure weave: {:#}", err);
            return ExitCode::FAILURE;
        }
    };

    let filter = if settings.verbose { "info" } else { "warn" };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)),
        )
        .init();

    match run(&settings) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{:#}", err);
            ExitCode::FAILURE
        }
    }
}
