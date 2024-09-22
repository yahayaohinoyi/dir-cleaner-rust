use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::ReportData;

use super::utils::{collect_metrics, delete_file};

fn get_file_key(file_name: Option<&std::ffi::OsStr>, file_size: u64) -> Option<String> {
    file_name.and_then(|name| name.to_str()).map(|name| {
        // Extract the first part of the file name, before any whitespace or special characters
        let first_part = name.split_whitespace().next().unwrap_or("");
        format!("{}{}", first_part, file_size)
    })
}

pub fn directory_cleaner_based_on_duplicate_files(
    directory: &String,
    dry_run: bool,
    report_data: &mut ReportData,
    paths_to_ignore: &[String],
) -> Result<()> {
    let mut set: HashSet<String> = HashSet::new();
    let mut del_count: u32 = 0;
    let mut del_size: u64 = 0;
    // Convert paths_to_ignore to a collection of PathBuf for easier comparison
    let ignore_set: HashSet<PathBuf> = paths_to_ignore.iter().map(PathBuf::from).collect();
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
                                delete_file(path, dry_run, &ignore_set)?;
                                del_count += 1;
                                del_size += metadata.len();
                            } else {
                                set.insert(file_key);
                            }
                        }

                        None => {}
                    }
                    collect_metrics(report_data, metadata, &path, (del_count, del_size));
                    del_count = 0;
                    del_size = 0;
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
