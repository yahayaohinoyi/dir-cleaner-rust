use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::ReportData;

use super::utils::{collect_metrics, delete_file};

pub fn directory_cleaner_based_on_file_size(
    directory: &String,
    size: u64,
    dry_run: bool,
    report_data: &mut ReportData,
    paths_to_ignore: &[String],
) -> Result<()> {
    let mut del_count: u32 = 0;
    let mut del_size: u64 = 0;
    // Convert paths_to_ignore to a collection of PathBuf for easier comparison
    let ignore_set: Vec<PathBuf> = paths_to_ignore.iter().map(PathBuf::from).collect();
    for entry in WalkDir::new(directory).into_iter() {
        match entry {
            Ok(dir) => {
                let path = dir.path();

                if ignore_set.iter().any(|ignore_path| path.starts_with(ignore_path)) {
                    println!("Skipping ignored path: {:?}", path);
                    continue; // Skip this path if it's in the ignore list
                }

                if path.is_file() {
                    let metadata = fs::metadata(path)
                        .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                    if metadata.len() >= size {
                        delete_file(path, dry_run)?;
                        del_count += 1;
                        del_size += metadata.len();
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
