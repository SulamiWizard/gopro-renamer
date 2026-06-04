use std::{
    fs::{DirEntry, rename},
    io,
};

use super::GoProFile;
use chrono::{DateTime, Local};
use colored::Colorize;

pub fn rename_file(file: &GoProFile, dry_run: bool, prefix: &str) -> io::Result<()> {
    let file_path = file.path.path();
    let new_file_name = get_new_name(file, prefix);

    if let Some(parent_dir) = file_path.parent() {
        let new_file_full_path = parent_dir.join(new_file_name);

        if dry_run {
            println!(
                "{} {} {} {}",
                "[DRY RUN]".yellow().bold(),
                file_path.file_name().unwrap().to_string_lossy().cyan(),
                "->".dimmed(),
                new_file_full_path.to_string_lossy().green()
            )
        } else {
            println!(
                "{} {} {} {}",
                "Renaming:".bold(),
                file_path.file_name().unwrap().to_string_lossy().cyan(),
                "->".dimmed(),
                new_file_full_path.to_string_lossy().green()
            );
            rename(file_path, new_file_full_path)?;
        }
    }
    Ok(())
}

fn get_new_name(file: &GoProFile, prefix: &str) -> String {
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
