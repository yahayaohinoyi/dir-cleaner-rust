use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::features::utils::delete_file;
use crate::ReportData;

use super::utils::collect_metrics;

pub fn directory_cleaner_based_on_file_type(
    dir: &String,
    types_to_clear: &[String],
    dry_run: bool,
    report_data: &mut ReportData,
    paths_to_ignore: &[String],
) -> Result<()> {
    let mut del_count: u32 = 0;
    let mut del_size: u64 = 0;

    // Convert paths_to_ignore to a collection of PathBuf for easier comparison
    let ignore_set: HashSet<PathBuf> = paths_to_ignore.iter().map(PathBuf::from).collect();
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

        if ignore_set
            .iter()
            .any(|ignore_path| path.starts_with(ignore_path))
        {
            println!("Skipping ignored path: {:?}", path);
            continue; // Skip this path if it's in the ignore list
        }

        if path.is_file() {
            let ext = path.extension().and_then(|ex| ex.to_str()).unwrap_or("");

            let metadata = fs::metadata(path)
                .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

            if types_to_clear.iter().any(|p| ext == p) {
                println!("Deleting file: {:?}", path);

                delete_file(path, dry_run, &ignore_set)?;
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::path::Path;
    use tempdir::TempDir;

    #[test]
    fn test_directory_cleaner_ignores_files() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new("test_dir").expect("Failed to create temp dir");

        // Setup test directory structure
        let files_dir = temp_dir.path().join("Files");
        let texts_dir = files_dir.join("Documents/texts");

        // Create directories
        fs::create_dir_all(&texts_dir).expect("Failed to create texts directory");

        // Create sample files
        let text_file_1 = texts_dir.join("text1.txt");
        let text_file_2 = texts_dir.join("text2.txt");

        // Create text files
        File::create(&text_file_1).expect("Failed to create text1 file");
        File::create(&text_file_2).expect("Failed to create text2 file");

        // Prepare parameters for the function
        let dir_to_clean = files_dir.to_str().unwrap().to_string();
        let types_to_clear = vec!["txt".to_string()]; // Clearing .txt files
        let dry_run = false;
        let mut report_data = ReportData::new();
        let files_to_ignore = vec!["text1.txt".to_string()]; // Ignore specific file

        // Run the cleaner function
        let result = directory_cleaner_based_on_file_type(
            &dir_to_clean,
            &types_to_clear,
            dry_run,
            &mut report_data,
            &files_to_ignore,
        );

        // Ensure the function ran successfully
        assert!(result.is_ok());

        // Check if the ignored file still exists
        assert!(
            Path::new(&text_file_1).exists(),
            "text1.txt should not be deleted"
        );

        // Check if the non-ignored file is deleted
        assert!(
            !Path::new(&text_file_2).exists(),
            "text2.txt should be deleted"
        );
    }
}
