use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use super::GoProFile;
use colored::Colorize;

pub fn concatenate_files(path: PathBuf, files: Vec<GoProFile>, dryrun: bool) {
    let mut hashfiles: HashMap<u16, Vec<GoProFile>> = HashMap::new();
    for file in files {
        let entry = hashfiles.entry(file.video_num).or_default();
        entry.push(file);
    }

    // Sort the files by chapter number because read_dir can read in an arbitrary order
    for files in hashfiles.values_mut() {
        files.sort_by_key(|f| f.chapter_num);
    }

    for (video_number, chapters) in hashfiles.iter() {
        if dryrun {
            println!("{}", "-".repeat(40).dimmed());
            println!("{} {}.mp4", "Output:".bold(), video_number);
            for chapter in chapters {
                println!(
                    "  {} {}",
                    "+".dimmed(),
                    chapter.path.file_name().to_string_lossy().dimmed()
                );
            }
            println!("{}", "-".repeat(40).dimmed());
        } else {
            let temp_file_path = create_temp_file(&path, video_number, chapters);

            run_concatenate_command(&path, video_number, &temp_file_path);
        }
    }
}

fn create_temp_file(path: &Path, video_number: &u16, chapters: &[GoProFile]) -> PathBuf {
    let concat_list = chapters
        .iter()
        .map(|f| {
            format!(
                "file '{}'",
                f.path.path().canonicalize().unwrap().to_string_lossy()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let temp_path = path.join(format!("{}_concat_list.txt", video_number));
    fs::write(&temp_path, concat_list).unwrap();
    temp_path
}

fn run_concatenate_command(path: &Path, video_number: &u16, temp_path: &PathBuf) {
    // Do the ffmpeg command to concatenate the videos
    let output_name = format!("{}.mp4", video_number);
    let output_path = path.join(output_name);

    // The command being run is
    // ffmpeg -f concat -safe 0 -i <temp_path> -c copy <output_path>
    // temp_path is just a .txt file with the paths to each video file in order
    // and output_path will be <video_number>.mp4
    // the -c makes it lossless but will only work with files with the exact same encoding
    // details, This should be a non issue because the GoPro files will be coming from the
    // same GoPro, so they should all have the same encoding.
    //
    // There are currently no plans to make this work with files that have mismatched encoding
    // settings
    std::process::Command::new("ffmpeg")
        .args(["-f", "concat", "-safe", "0", "-i"])
        .arg(temp_path)
        .args(["-c", "copy"])
        .arg(&output_path)
        .status()
        .unwrap();

    // remove the temp file
    fs::remove_file(temp_path).unwrap();
}
