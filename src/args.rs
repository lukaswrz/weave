use std::{ffi::OsString, path::PathBuf};

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple link manager")]
pub(crate) struct Args {
    /// The path where the new links should be placed
    #[arg(short = 't', long = "to", env = "WEAVE_TO")]
    pub(crate) to: Option<PathBuf>,

    /// Whether files in the destination should forcibly be replaced instead of being skipped
    #[arg(short = 'F', long = "force", action = ArgAction::SetTrue)]
    pub(crate) force: bool,

    /// Decide what to do interactively when a conflict occurs
    #[arg(short = 'i', long = "interactive", action = ArgAction::SetTrue)]
    pub(crate) interactive: bool,

    /// Print verbose output
    #[arg(short = 'v', long = "verbose", action = ArgAction::SetTrue)]
    pub(crate) verbose: bool,

    /// Restrict the mode of the directories that will be created
    #[arg(short = 'm', long = "directory-mode", value_parser = parse_permissions, default_value = "777")]
    pub(crate) directory_mode: u32,

    /// The path to the configuration file
    #[arg(short = 'c', long = "config", default_value = "weave.toml")]
    pub(crate) config: PathBuf,

    /// What should be linked
    pub(crate) choices: Vec<OsString>,
}

fn parse_permissions(string: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(string, 8)
}
