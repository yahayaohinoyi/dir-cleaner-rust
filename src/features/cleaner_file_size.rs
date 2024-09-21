use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;

use crate::ReportData;

use super::utils::delete_file;

pub fn directory_cleaner_based_on_file_size(
    directory: &String,
    size: u64,
    dry_run: bool,
    report_data: &mut ReportData
) -> Result<()> {
    for entry in WalkDir::new(directory).into_iter() {
        match entry {
            Ok(dir) => {
                let path = dir.path();

                if path.is_file() {
                    let metadata = fs::metadata(path)
                        .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                    if metadata.len() >= size {
                        delete_file(path, dry_run)?;
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
