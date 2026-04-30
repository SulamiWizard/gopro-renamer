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

struct GoProFile {
    path: DirEntry,
    video_num: u8,
    chapter_num: u8,
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
        concatenate_files(path, files, args.dry_run);
    } else {
        for file in files {
            let _ = rename_file(&file, args.dry_run, &custom_prefix);
        }
    }
}

fn get_files(path: &Path) -> Vec<GoProFile> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
            if let Some(gpfile) = is_gopro_file(file) {
                data.push(gpfile);
            }
        }
    }
    data
}

fn is_gopro_file(file: DirEntry) -> Option<GoProFile> {
    // Check if file name matches the GoPro chaptered file naming scheme and create a GoProFile
    // struct and return it, otherwise return None
    let re = Regex::new(r"^(G[HX])([0-9]{2})([0-9]{4})\.MP4$").unwrap();
    let file_name = file.file_name().to_str().unwrap().to_string();
    let mut goprofile: Option<GoProFile> = None;

    if let Some(captures) = re.captures(file_name.as_str()) {
        // let encoding = &captures[1];
        let chapter_number = &captures[2];
        let video_number = &captures[3];
        goprofile = Some(GoProFile {
            path: file,
            video_num: video_number.parse().unwrap_or_default(),
            chapter_num: chapter_number.parse().unwrap_or_default(),
        })
        // Create new file name using these captures
    }
    goprofile
}

fn rename_file(file: &GoProFile, dry_run: bool, prefix: &String) -> io::Result<()> {
    let file_path = file.path.path();
    let new_file_name = get_new_name(file, prefix);

    if let Some(parent_dir) = file_path.parent() {
        let new_file_full_path = parent_dir.join(new_file_name);

        if dry_run {
            println!(
                "Would rename: {} -> {}",
                file_path.to_string_lossy(),
                new_file_full_path.to_string_lossy()
            )
        } else {
            rename(file_path, new_file_full_path)?;
        }
    }
    Ok(())
}

fn get_new_name(file: &GoProFile, prefix: &String) -> String {
    let date_string: String;

    let new_prefix: &str = if prefix == "%DATE" {
        date_string = format!("{}_", get_date(&file.path).unwrap_or_default());
        &date_string
    } else {
        prefix
    };

    format!(
        "{}{}_CH{}.MP4",
        new_prefix, file.video_num, file.chapter_num
    )
}

fn get_date(file: &DirEntry) -> io::Result<String> {
    let metadata = file.metadata()?;
    let modified: DateTime<Local> = metadata.modified()?.into();

    Ok(modified.format("%Y-%m-%d").to_string())
}

fn concatenate_files(path: PathBuf, files: Vec<GoProFile>, dryrun: bool) {
    let mut hashfiles: HashMap<u8, Vec<GoProFile>> = HashMap::new();
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
            println!("Will combine files:");
            for chapter in chapters {
                println!("{}", chapter.path.file_name().to_string_lossy());
            }
            println!("As {}.MP4", video_number);
            println!();
        } else {
            let temp_file_path = create_temp_file(&path, video_number, chapters);

            run_concatenate_command(&path, video_number, &temp_file_path);
        }
    }
}

fn create_temp_file(path: &Path, video_number: &u8, chapters: &[GoProFile]) -> PathBuf {
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

fn run_concatenate_command(path: &Path, video_number: &u8, temp_path: &PathBuf) {
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
