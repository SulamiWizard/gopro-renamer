use chrono::{DateTime, Local};
use clap::Parser;
use regex::Regex;
use std::{
    fs::{self, DirEntry, rename},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Rename chaptered GoPro video files")]
struct Args {
    path: Option<PathBuf>,

    #[arg(short = 'd', long = "dry-run")]
    dry_run: bool,

    /// Prefix to add to renamed files.
    /// Use '$DATE' to prefix with the file's modified date (e.g. '2026-04-27').
    #[arg(short = 'p', long = "prefix", default_value_t = String::from(""))]
    prefix: String,
}

fn main() {
    let args = Args::parse();
    // check if directory is a valid directory, if not, use cwd
    let path = args.path.unwrap_or(PathBuf::from("."));
    let files = get_files(&path);

    let custom_prefix = args.prefix;

    for file in files {
        let _ = rename_file(&file, args.dry_run, &custom_prefix);
    }
}

fn get_files(path: &Path) -> Vec<DirEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
            // check if file ends with .mp4 then push it
            if check_file_extension(&file, ".mp4") {
                data.push(file);
            }
        }
    }
    data
}

fn check_file_extension(file: &DirEntry, ext: &str) -> bool {
    file.file_name()
        .to_str()
        .unwrap()
        .to_lowercase()
        .ends_with(ext)
}

fn rename_file(file: &DirEntry, dry_run: bool, prefix: &String) -> io::Result<()> {
    let file_path = file.path();
    let new_file_name = get_new_name(file, prefix);

    if let Some(parent_dir) = file_path.parent() {
        let new_file_full_path = parent_dir.join(new_file_name);

        if dry_run {
            println!(
                "Would rename: {} -> {}",
                file.path().to_string_lossy(),
                new_file_full_path.to_string_lossy()
            )
        } else {
            rename(file.path(), new_file_full_path)?;
        }
    }
    Ok(())
}

fn get_new_name(file: &DirEntry, prefix: &String) -> String {
    let re = Regex::new(r"^(G[HX])([0-9]{2})([0-9]{4})\.MP4$").unwrap();
    let file_name = file.file_name().to_str().unwrap().to_string();

    let mut new_name: String = file_name.to_owned();

    let date_string: String;
    let new_prefix: &str = if prefix == "%DATE" {
        date_string = format!("{}_", get_date(file).unwrap_or_default());
        &date_string
    } else {
        prefix
    };

    if let Some(captures) = re.captures(file_name.as_str()) {
        let encoding = &captures[1];
        let chapter_number = &captures[2];
        let video_number = &captures[3];
        // Create new file name using these captures
        new_name = format!(
            "{}{}_{}_CH{}.MP4",
            new_prefix, encoding, video_number, chapter_number
        );
    }
    new_name
}

fn get_date(file: &DirEntry) -> io::Result<String> {
    let metadata = file.metadata()?;
    let modified: DateTime<Local> = metadata.modified()?.into();

    Ok(modified.format("%Y-%m-%d").to_string())
}
