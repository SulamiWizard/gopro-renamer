use std::path::PathBuf;

use crate::cli::{self, args::Args};

use colored::Colorize;
use text_io::read;

pub fn run() {
    // Fill Args struct fields with user input
    print!("{}", "enter Path: ".yellow());
    let path_ans: String = read!("{}\n");
    let path = PathBuf::from(path_ans).into();

    print!("{}", "dry run(true/false): ".yellow());
    let dry_run: bool = read!("{}\n");

    print!("{}", "prefix: ".yellow());
    let prefix: String = read!("{}\n");

    print!("{}", "concat(true/false): ".yellow());
    let concatenate: bool = read!("{}\n");

    let args = Args {
        path,
        dry_run,
        prefix,
        concatenate,
    };

    cli::run(args);
}
