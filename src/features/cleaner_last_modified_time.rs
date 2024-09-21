use crate::{features::utils, ReportData};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs;
use walkdir::WalkDir;

use super::utils::{collect_metrics, delete_file};

pub fn directory_cleaner_based_on_age(
    directory: &String,
    date: String,
    dry_run: bool,
    report_data: &mut ReportData,
) -> Result<()> {
    let mut del_count: u32 = 0;
    let mut del_size: u64 = 0;
    for entry in WalkDir::new(directory).into_iter() {
        match entry {
            Ok(dir) => {
                let path = dir.path();

                if path.is_file() {
                    let metadata = fs::metadata(path)
                        .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;
                    if let Ok(modified_time) = metadata.modified() {
                        let modified_time_utc: DateTime<Utc> = modified_time.into();
                        let cutoff_date = utils::parse_cutoff_date(&date)?;
                        if modified_time_utc < cutoff_date {
                            delete_file(path, dry_run)?;
                            del_count += 1;
                            del_size += metadata.len();
                        }
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
