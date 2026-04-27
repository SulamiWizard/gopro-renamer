use clap::Parser;
use regex::Regex;
use std::{
    fs::{self, DirEntry, rename},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Rename chaptered GoPro video files")]
struct CliArgs {
    path: Option<PathBuf>,
}

fn main() {
    let args = CliArgs::parse();
    // check if directory is a valid directory, if not, use cwd
    let path = args.path.unwrap_or(PathBuf::from("."));
    // if path.is_dir() {
    //     println!("is dir");
    // }
    // for file in path
    let files = get_files(&path);
    for file in files {
        // println!("{:?}", file);
        let _ = rename_file(&file);
    }
    // rename the file
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

fn rename_file(file: &DirEntry) -> io::Result<()> {
    let file_path = file.path();
    let new_file_name = get_new_name(file.file_name().to_str().unwrap());

    if let Some(parent_dir) = file_path.parent() {
        let new_file_full_path = parent_dir.join(new_file_name);

        rename(file.path(), new_file_full_path)?;
    }
    Ok(())
}

fn get_new_name(file_name: &str) -> String {
    let re = Regex::new(r"^(G[HX])([0-9]{2})([0-9]{4})\.MP4$").unwrap();
    let mut new_name: String = String::from(file_name);

    if let Some(captures) = re.captures(file_name) {
        let encoding = &captures[1];
        let chapter_number = &captures[2];
        let video_number = &captures[3];
        // Create new file name using these captures
        new_name = format!("{}_{}_CH{}.MP4", encoding, video_number, chapter_number);
    }
    new_name
}
