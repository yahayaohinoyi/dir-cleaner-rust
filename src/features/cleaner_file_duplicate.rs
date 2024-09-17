use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use walkdir::WalkDir;

use super::utils::delete_file;

fn get_file_key(file_name: Option<&std::ffi::OsStr>, file_size: u64) -> Option<String> {
    file_name.and_then(|name| name.to_str()).map(|name| {
        // Extract the first part of the file name, before any whitespace or special characters
        let first_part = name.split_whitespace().next().unwrap_or("");
        format!("{}{}", first_part, file_size)
    })
}

pub fn directory_cleaner_based_on_duplicate_files(directory: &String, dry_run: bool) -> Result<()> {
    let mut set: HashSet<String> = HashSet::new();
    for entry in WalkDir::new(directory).into_iter() {
        match entry {
            Ok(dir) => {
                let path = dir.path();

                if path.is_file() {
                    let metadata = fs::metadata(path)
                        .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                    // use first word in filename + filesize as they key till we think of something
                    // better

                    let file_key = get_file_key(path.file_name(), metadata.len());

                    match file_key {
                        Some(file_key) => {
                            if set.contains(&file_key) {
                                delete_file(path, dry_run)?;
                            } else {
                                set.insert(file_key);
                            }
                        }

                        None => {}
                    }
                } else {
                    eprintln!("File does not exist, {}", path.display())
                }
            }

            Err(err) => {
                eprintln!(
                    "Encountered error trying to get file. operation proceeding..., {}",
                    err
                );
            }
        }
    }

    Ok(())
}
