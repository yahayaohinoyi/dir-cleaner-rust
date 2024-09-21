use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::features::utils::delete_file;
use crate::ReportData;

use super::utils::collect_metrics;

pub fn directory_cleaner_based_on_file_type(
    dir: &String,
    paths_to_clear: &[String],
    dry_run: bool,
    report_data: &mut ReportData,
) -> Result<()> {
    let mut del_count: u32 = 0;
    let mut del_size: u32 = 0;
    for entry in WalkDir::new(dir).into_iter().filter_map(|f| {
        match f {
            Ok(entry) => Some(entry), // Return valid entries
            Err(err) => {
                eprintln!(
                    "Error accessing file: {}, Error: {}",
                    err.path().unwrap_or_else(|| Path::new("unknown")).display(),
                    err
                );
                None // Skip erroneous entries
            }
        }
    }) {
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension().and_then(|ex| ex.to_str()).unwrap_or("");

            let metadata = fs::metadata(path)
                .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;
            if paths_to_clear.iter().any(|p| ext == p) {
                println!("Deleting file: {:?}", path);

                delete_file(path, dry_run)?;
                del_count += 1;
                del_size += metadata.len() as u32;
            }
            collect_metrics(report_data, metadata, &path, (del_count, del_size));
            del_count = 0;
            del_size = 0;
        } else {
            eprintln!("File does not exist, {}", path.display())
        }
    }

    Ok(())
}
