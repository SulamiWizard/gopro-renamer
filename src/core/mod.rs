use std::{
    fs::{self, DirEntry},
    path::Path,
};

use regex::Regex;

pub mod concatenate;
pub mod rename;

pub struct GoProFile {
    pub path: DirEntry,
    pub video_num: u16,
    pub chapter_num: u8,
}

pub fn get_files(path: &Path) -> Vec<GoProFile> {
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
