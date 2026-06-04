use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Rename chaptered GoPro video files")]
pub struct Args {
    /// Path to directory containing GoPro Video Files
    pub path: Option<PathBuf>,

    /// Do a dry run of the program, print out files that would be changed
    /// without renaming them
    #[arg(short = 'd', long = "dry-run")]
    pub dry_run: bool,

    /// Prefix to add to renamed files.
    /// Use '%DATE' to prefix with the file's modified date (e.g. '2026-04-27').
    #[arg(short = 'p', long = "prefix", default_value_t = String::from(""))]
    pub prefix: String,

    /// Compine chaptered video files into 1 video file per video number
    #[arg(short = 'c', long = "concatenate-videos")]
    pub concatenate: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
