pub mod args;
use std::path::PathBuf;

use super::core;
use super::core::concatenate::concatenate_files;
use super::core::rename::rename_file;

use args::Args;
use colored::Colorize;

pub fn run(args: Args) {
    // check if directory is a valid directory, if not, use cwd
    let path = args.path.unwrap_or(PathBuf::from("."));
    let files = core::get_files(&path);

    if files.is_empty() {
        eprintln!(
            "{}",
            "No GoPro files found in the specified directory.".yellow()
        );
        return;
    }

    let custom_prefix = args.prefix;

    if args.concatenate {
        // If the concatenate flag is true, rather than rename the files, we will sort the files
        // into a hashmap with the video number being the key, then concatenate the files associated
        // with each key together
        concatenate_files(path, files, args.dry_run);
    } else {
        let mut count: usize = 0;
        for file in files {
            if rename_file(&file, args.dry_run, &custom_prefix).is_ok() {
                count += 1;
            }
        }
        if !args.dry_run {
            println!("\n{} {} file(s) renamed.", "Done!".green().bold(), count);
        }
    }
}
