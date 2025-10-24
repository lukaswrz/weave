use std::{ffi::OsString, fs, path::PathBuf};

use crate::args;
use anyhow::{Error, Result, anyhow};
use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ConfigFile {
    from: String,
    to: String,
}

pub(crate) struct Settings {
    pub(crate) from: PathBuf,
    pub(crate) to: PathBuf,
    pub(crate) force: bool,
    pub(crate) interactive: bool,
    pub(crate) verbose: bool,
    pub(crate) directory_mode: u32,
    pub(crate) choices: Vec<OsString>,
}

fn parse_path(path: &str) -> Result<PathBuf> {
    match shellexpand::full(path) {
        Ok(i) => Ok(PathBuf::from(i.into_owned())),
        Err(e) => Err(anyhow!("unable to parse path: {}", e)),
    }
}

pub(crate) fn configure() -> Result<Settings, Error> {
    let args = args::Args::parse();

    let config_string = match fs::read_to_string(&args.config) {
        Ok(s) => s,
        Err(e) => {
            return Err(anyhow!("unable to read config file: {}", e));
        }
    };
    let cf: ConfigFile = toml::from_str(&config_string)?;

    Ok(Settings {
        from: parse_path(&cf.from)?,
        to: match args.to {
            Some(t) => t,
            None => parse_path(&cf.to)?,
        },
        force: args.force,
        interactive: args.interactive,
        verbose: args.verbose,
        directory_mode: args.directory_mode,
        choices: args.choices,
    })
}
