use chrono::{DateTime, Local};
use clap::Parser;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, DirEntry, rename},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Rename chaptered GoPro video files")]
struct Args {
    /// Path to directory containing GoPro Video Files
    path: Option<PathBuf>,

    /// Do a dry run of the program, print out files that would be changed
    /// without renaming them
    #[arg(short = 'd', long = "dry-run")]
    dry_run: bool,

    /// Prefix to add to renamed files.
    /// Use '%DATE' to prefix with the file's modified date (e.g. '2026-04-27').
    #[arg(short = 'p', long = "prefix", default_value_t = String::from(""))]
    prefix: String,

    /// Compine chaptered video files into 1 video file per video number
    #[arg(short = 'c', long = "concatenate-videos")]
    concatenate: bool,
}

fn main() {
    let args = Args::parse();
    // check if directory is a valid directory, if not, use cwd
    let path = args.path.unwrap_or(PathBuf::from("."));
    let files = get_files(&path);

    let custom_prefix = args.prefix;

    if args.concatenate {
        // If the concatenate flag is true, rather than rename the files, we will sort the files
        // into a hashmap with the video number being the key, then concatenate the files associated
        // with each key together
        concatenate_files(path, files);
    } else {
        for file in files {
            let _ = rename_file(&file, args.dry_run, &custom_prefix);
        }
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
        // let encoding = &captures[1];
        let chapter_number = &captures[2];
        let video_number = &captures[3];
        // Create new file name using these captures
        new_name = format!("{}{}_CH{}.MP4", new_prefix, video_number, chapter_number);
    }
    new_name
}

fn get_date(file: &DirEntry) -> io::Result<String> {
    let metadata = file.metadata()?;
    let modified: DateTime<Local> = metadata.modified()?.into();

    Ok(modified.format("%Y-%m-%d").to_string())
}

fn get_file_number(file: &DirEntry) -> String {
    let re = Regex::new(r"^(G[HX])([0-9]{2})([0-9]{4})\.MP4$").unwrap();
    let file_name = file.file_name().to_str().unwrap().to_string();

    let mut new_name: String = String::from("");

    if let Some(captures) = re.captures(file_name.as_str()) {
        let video_number = &captures[3];
        // new_name = format!("{}", video_number);
        new_name = video_number.to_string();
    }
    new_name
}

fn get_chapter_number(file: &DirEntry) -> u8 {
    let re = Regex::new(r"^(G[HX])([0-9]{2})([0-9]{4})\.MP4$").unwrap();
    let file_name = file.file_name().to_str().unwrap().to_string();

    let mut chapter: u8 = 0;

    if let Some(captures) = re.captures(file_name.as_str()) {
        let chapter_number = &captures[2];
        // new_name = format!("{}", video_number);
        chapter = chapter_number.to_string().parse().unwrap();
    }
    chapter
}

fn concatenate_files(path: PathBuf, files: Vec<DirEntry>) {
    let mut hashfiles: HashMap<String, Vec<DirEntry>> = HashMap::new();
    for file in files {
        let entry = hashfiles.entry(get_file_number(&file)).or_default();
        entry.push(file);
    }

    // Sort the files by chapter number because read_dir can read in an arbitrary order
    for files in hashfiles.values_mut() {
        files.sort_by_key(get_chapter_number);
    }

    // TODO: add dry run functionality
    // Create file for ffmpeg to use for concatenating videos
    for (video_number, chapters) in hashfiles.iter() {
        let temp_file_path = create_temp_file(&path, video_number, chapters);

        run_concatenate_command(&path, video_number, &temp_file_path);
    }
}

fn create_temp_file(path: &Path, video_number: &String, chapters: &[DirEntry]) -> PathBuf {
    let concat_list = chapters
        .iter()
        .map(|f| {
            format!(
                "file '{}'",
                f.path().canonicalize().unwrap().to_string_lossy()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let temp_path = path.join(format!("{}_concat_list.txt", video_number));
    fs::write(&temp_path, concat_list).unwrap();
    temp_path
}

fn run_concatenate_command(path: &Path, video_number: &String, temp_path: &PathBuf) {
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
    // There are currently no plans to make this work with mismatched files, but i'll
    // implement if it is necessary
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
