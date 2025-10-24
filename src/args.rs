use std::{ffi::OsString, path::PathBuf};

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple link manager")]
pub(crate) struct Args {
    #[arg(short = 't', long = "to", env = "WEAVE_TO")]
    pub(crate) to: Option<PathBuf>,

    #[arg(short = 'F', long = "force", action = ArgAction::SetTrue)]
    pub(crate) force: bool,

    #[arg(short = 'i', long = "interactive", action = ArgAction::SetTrue)]
    pub(crate) interactive: bool,

    #[arg(short = 'v', long = "verbose", action = ArgAction::SetTrue)]
    pub(crate) verbose: bool,

    #[arg(short = 'm', long = "directory-mode", value_parser = parse_permissions, default_value = "777")]
    pub(crate) directory_mode: u32,

    #[arg(long, default_value = "weave.toml")]
    pub(crate) config: PathBuf,

    pub(crate) choices: Vec<OsString>,
}

fn parse_permissions(string: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(string, 8)
}
