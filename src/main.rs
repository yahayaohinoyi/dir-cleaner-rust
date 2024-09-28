use anyhow::Result;
use colored::*;
use features::utils::read_file_and_rebuild_args;
use std::{collections::HashSet, time::Instant};
mod arg;
mod features;

#[derive(Debug)]
struct ReportData {
    files_scanned: u32,
    files_deleted: u32,
    total_time_sec: u64,
    total_file_size_deleted: u64,
    total_file_size_retained: u64,
    total_files_retained: u32,
    paths_deleted: Vec<String>,
    paths_retained: Vec<String>,
}

impl ReportData {
    fn new() -> Self {
        ReportData {
            files_scanned: 0,
            files_deleted: 0,
            total_time_sec: 0,
            total_file_size_deleted: 0,
            total_file_size_retained: 0,
            total_files_retained: 0,
            paths_deleted: vec![],
            paths_retained: vec![],
        }
    }

    fn print_report(&self) {
        // Section headers with bold and different colors
        println!("{}", "Cleaning Report".bold().underline().blue());
        println!("");

        // Files scanned
        println!(
            "{}: {}",
            "Files Scanned".bold().cyan(),
            self.files_scanned.to_string().green()
        );

        // Files deleted
        println!(
            "{}: {}",
            "Files Deleted".bold().cyan(),
            self.files_deleted.to_string().red()
        );

        // Files retained
        println!(
            "{}: {}",
            "Files Retained".bold().cyan(),
            self.total_files_retained.to_string().yellow()
        );

        // Time taken in seconds
        println!(
            "{}: {}",
            "Total Time (seconds)".bold().cyan(),
            self.total_time_sec.to_string().magenta()
        );

        // File sizes deleted
        println!(
            "{}: {} bytes",
            "Total File Size Deleted".bold().cyan(),
            self.total_file_size_deleted.to_string().red()
        );

        // File sizes retained
        println!(
            "{}: {} bytes",
            "Total File Size Retained".bold().cyan(),
            self.total_file_size_retained.to_string().yellow()
        );

        // Paths of deleted files (if any)
        if !self.paths_deleted.is_empty() {
            println!("\n{}", "Paths Deleted".bold().red());
            for path in &self.paths_deleted {
                println!("{}", path.red());
            }
        }

        // Paths of retained files (if any)
        if !self.paths_retained.is_empty() {
            println!("\n{}", "Paths Retained".bold().yellow());
            let paths_retained_unique: HashSet<_> = self.paths_retained.iter().cloned().collect();

            for path in paths_retained_unique {
                println!("{}", path.yellow());
            }
        }
    }
}

fn main() -> Result<()> {
    let mut args = arg::parse_args();

    let mut report_data = ReportData::new();
    let start = Instant::now();

    if args.config_file.is_some() {
        read_file_and_rebuild_args(&mut args)?;
    }

    if args.dry_run {
        println!("{}", "=== Dry Run Report ===".bold().underline().cyan());
        println!(
            "{}",
            "This report provides an overview of what could have been deleted."
                .italic()
                .dimmed()
        );
    }

    if args.types.len() > 0 {
        println!("Cleaning directory based on file type: {:?}", args.dir);
        println!("File types to clean: {:?}", args.types);
        features::cleaner_file_type::directory_cleaner_based_on_file_type(
            &args.dir,
            &args.types,
            args.dry_run,
            &mut report_data,
            &args.files_to_ignore,
        )?;
    }
    if let Some(val) = args.min_size {
        println!("Cleaning directory based on min size: {:?}", args.dir);
        println!("Minimum file size: {} bytes", val);
        features::cleaner_file_size::directory_cleaner_based_on_file_size(
            &args.dir,
            val,
            args.dry_run,
            &mut report_data,
            &args.files_to_ignore,
        )?;
    }
    if args.remove_duplicates {
        println!(
            "Cleaning directory based on duplicate files: {:?}",
            args.dir
        );
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &args.dir,
            args.dry_run,
            &mut report_data,
            &args.files_to_ignore,
        )?;
    }
    if let Some(age_value) = args.age {
        println!("Cleaning directory based on age: {:?}", age_value);
        features::cleaner_last_modified_time::directory_cleaner_based_on_age(
            &args.dir,
            age_value,
            args.dry_run,
            &mut report_data,
            &args.files_to_ignore,
        )?;
    }

    println!("Cleaning completed successfully.");
    let duration = start.elapsed();
    report_data.total_time_sec = duration.as_secs();

    // show report
    report_data.print_report();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_directory_cleaner_should_delete_files_of_the_specified_file_type() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.txt");
        let file = File::create(&file_path)?;
        file.set_len(1000)?; // 1000 bytes

        let mut report = ReportData::new();

        let file_types = vec!["txt".to_string()];
        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_type::directory_cleaner_based_on_file_type(
            &dir_str,
            &file_types,
            false,
            &mut report,
            &[],
        )?;

        assert!(!file_path.exists(), "File should have been deleted");

        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_not_delete_files_of_the_specified_file_type_in_dry_run_mode(
    ) -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.txt");
        let file = File::create(&file_path)?;
        file.set_len(1000)?; // 1000 bytes

        let mut report = ReportData::new();

        let file_types = vec!["txt".to_string()];
        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_type::directory_cleaner_based_on_file_type(
            &dir_str,
            &file_types,
            true,
            &mut report,
            &[],
        )?;

        assert!(file_path.exists(), "File shouldn't be deleted in dry run");

        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_delete_files_greater_than_the_specified_min_size() -> Result<()>
    {
        let temp_dir = tempdir()?;
        let file_path_1 = temp_dir.path().join("test1.txt");
        let file_path_2 = temp_dir.path().join("test2.txt");
        let file_1 = File::create(&file_path_1)?;
        let file_2 = File::create(&file_path_2)?;
        file_1.set_len(4000)?; // 4000 bytes
        file_2.set_len(500)?; // 500 bytes

        let mut report = ReportData::new();

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_size::directory_cleaner_based_on_file_size(
            &dir_str,
            2000,
            false,
            &mut report,
            &[],
        )?;

        assert!(
            !file_path_1.exists(),
            "File should be deleted as it's more than the minimum size"
        );

        assert!(
            file_path_2.exists(),
            "File should not be deleted as it's less than the minimum size"
        );

        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_not_delete_files_greater_than_the_specified_min_size_in_dry_run_mode(
    ) -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path_1 = temp_dir.path().join("test1.txt");
        let file_path_2 = temp_dir.path().join("test2.txt");
        let file_1 = File::create(&file_path_1)?;
        let file_2 = File::create(&file_path_2)?;
        file_1.set_len(4000)?; // 4000 bytes
        file_2.set_len(500)?; // 500 bytes

        let mut report = ReportData::new();

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_size::directory_cleaner_based_on_file_size(
            &dir_str,
            2000,
            true,
            &mut report,
            &[],
        )?;

        assert!(
            file_path_1.exists(),
            "File should not be deleted in dry run mode"
        );

        assert!(
            file_path_2.exists(),
            "File should not be deleted in dry run mode"
        );

        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_delete_files_that_are_duplicates() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path_1 = temp_dir.path().join("test 1.txt");
        let file_path_2 = temp_dir.path().join("test 2.txt");
        let file_path_3 = temp_dir.path().join("test 3.txt");
        let file_1 = File::create(&file_path_1)?;
        let file_2 = File::create(&file_path_2)?;
        let file_3 = File::create(&file_path_3)?;
        file_1.set_len(4000)?; // 4000 bytes
        file_2.set_len(4000)?; // 4000 bytes
        file_3.set_len(5000)?; // 5000 bytes

        let mut report = ReportData::new();

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str,
            false,
            &mut report,
            &[],
        )?;

        assert!(
            !(file_path_1.exists() && file_path_2.exists()),
            "One of test 1 or test 2 should be deleted"
        );

        assert!(file_path_3.exists(), "test 3 shouldn't be deleted");

        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_not_delete_files_that_are_duplicates_in_dry_run() -> Result<()>
    {
        let temp_dir = tempdir()?;
        let file_path_1 = temp_dir.path().join("test 1.txt");
        let file_path_2 = temp_dir.path().join("test 2.txt");
        let file_path_3 = temp_dir.path().join("test 3.txt");
        let file_1 = File::create(&file_path_1)?;
        let file_2 = File::create(&file_path_2)?;
        let file_3 = File::create(&file_path_3)?;
        file_1.set_len(4000)?; // 4000 bytes
        file_2.set_len(4000)?; // 4000 bytes
        file_3.set_len(5000)?; // 5000 bytes

        let mut report = ReportData::new();

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str,
            true,
            &mut report,
            &[],
        )?;

        assert!(
            file_path_1.exists() && file_path_2.exists() && file_path_3.exists(),
            "All files should still exist"
        );

        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_delete_files_within_file_limit_and_are_duplicates(
    ) -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path_1 = temp_dir.path().join("test 1.txt");
        let file_path_2 = temp_dir.path().join("test 2.txt");
        let file_path_3 = temp_dir.path().join("test 3.txt");
        let file_1 = File::create(&file_path_1)?;
        let file_2 = File::create(&file_path_2)?;
        let file_3 = File::create(&file_path_3)?;
        file_1.set_len(4000)?; // 4000 bytes
        file_2.set_len(4000)?; // 4000 bytes
        file_3.set_len(5000)?; // 5000 bytes

        let mut report = ReportData::new();

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str,
            false,
            &mut report,
            &[],
        )?;

        assert!(
            !(file_path_1.exists() && file_path_2.exists()),
            "One of test 1 or test 2 should be deleted"
        );

        features::cleaner_file_size::directory_cleaner_based_on_file_size(
            &dir_str,
            4500,
            false,
            &mut report,
            &[],
        )?;

        assert!(!file_path_3.exists(), "test 3 should now be deleted");

        assert!(
            file_path_1.exists() || file_path_2.exists(),
            "One of test 1 or test 2 should still exist"
        );
        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_not_delete_files_within_file_limit_and_are_duplicates_in_dry_run(
    ) -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path_1 = temp_dir.path().join("test 1.txt");
        let file_path_2 = temp_dir.path().join("test 2.txt");
        let file_path_3 = temp_dir.path().join("test 3.txt");
        let file_1 = File::create(&file_path_1)?;
        let file_2 = File::create(&file_path_2)?;
        let file_3 = File::create(&file_path_3)?;
        file_1.set_len(4000)?; // 4000 bytes
        file_2.set_len(4000)?; // 4000 bytes
        file_3.set_len(5000)?; // 5000 bytes

        let mut report = ReportData::new();

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str,
            true,
            &mut report,
            &[],
        )?;

        assert!(
            file_path_1.exists() && file_path_2.exists() && file_path_3.exists(),
            "All files should still exist"
        );

        features::cleaner_file_size::directory_cleaner_based_on_file_size(
            &dir_str,
            4500,
            true,
            &mut report,
            &[],
        )?;

        assert!(
            file_path_1.exists() && file_path_2.exists() && file_path_3.exists(),
            "All files should still exist"
        );
        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_show_correct_report_after_deleting_files_greater_than_the_specified_min_size(
    ) -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path_1 = temp_dir.path().join("test1.txt");
        let file_path_2 = temp_dir.path().join("test2.txt");
        let file_1 = File::create(&file_path_1)?;
        let file_2 = File::create(&file_path_2)?;
        file_1.set_len(4000)?; // 4000 bytes
        file_2.set_len(500)?; // 500 bytes

        let mut report = ReportData::new();

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_size::directory_cleaner_based_on_file_size(
            &dir_str,
            2000,
            false,
            &mut report,
            &[],
        )?;

        assert!(
            !file_path_1.exists(),
            "File should be deleted as it's more than the minimum size"
        );

        assert!(
            file_path_2.exists(),
            "File should not be deleted as it's less than the minimum size"
        );

        assert!(report.files_deleted == 1);
        assert!(report.files_scanned == 2);
        assert!(report.total_files_retained == 1);
        assert!(report.total_file_size_retained == 500);
        assert!(report.total_file_size_deleted == 4000);
        assert!(report.paths_deleted.len() == 1);
        assert!(report.paths_retained.len() == 1);
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use chrono::{DateTime, Utc};
        use filetime::{set_file_times, FileTime};
        use std::fs;
        use std::path::Path;
        use tempfile::tempdir;

        fn set_file_modification_time(path: &Path, datetime: DateTime<Utc>) {
            let timestamp = datetime.timestamp();
            let file_time = FileTime::from_unix_time(timestamp, 0);
            set_file_times(path, file_time, file_time).expect("Failed to set file times");
        }

        #[test]
        fn test_files_older_than_cutoff_date() {
            let dir = tempdir().unwrap();
            let dir_path = dir.path().to_str().unwrap().to_string();

            // Create test files
            let file_path_older = dir.path().join("older_file.txt");
            let file_path_newer = dir.path().join("newer_file.txt");

            // Set cutoff date to 1 day ago
            let cutoff_date = Utc::now() - chrono::Duration::days(1);
            let cutoff_date_str = cutoff_date.format("%Y-%m-%d").to_string();

            // Create files
            fs::write(&file_path_older, "older file content").unwrap();
            fs::write(&file_path_newer, "newer file content").unwrap();

            // Set file modification times (older and newer than the cutoff date)
            set_file_modification_time(&file_path_older, cutoff_date - chrono::Duration::days(2)); // Older than cutoff
            set_file_modification_time(&file_path_newer, cutoff_date + chrono::Duration::days(2)); // Newer than cutoff

            let mut report = ReportData::new();

            // Run directory cleaner
            let result = features::cleaner_last_modified_time::directory_cleaner_based_on_age(
                &dir_path,
                cutoff_date_str,
                false, // not a dry run, actually delete files
                &mut report,
                &[],
            );

            assert!(result.is_ok());
            assert!(!file_path_older.exists()); // file should be deleted
            assert!(file_path_newer.exists()); // file should not be deleted
        }

        #[test]
        fn test_dry_run_mode_files_older_than_cutoff_date() {
            let dir = tempdir().unwrap();
            let dir_path = dir.path().to_str().unwrap().to_string();

            // Create a test file
            let file_path = dir.path().join("test_file.txt");
            fs::write(&file_path, "test content").unwrap();

            // Set cutoff date to 1 day ago
            let cutoff_date = Utc::now() - chrono::Duration::days(1);
            let cutoff_date_str = cutoff_date.format("%Y-%m-%d").to_string();

            // Set file modification time (older than cutoff)
            set_file_modification_time(&file_path, cutoff_date - chrono::Duration::days(2));

            let mut report = ReportData::new();

            // Run directory cleaner in dry run mode
            let result = features::cleaner_last_modified_time::directory_cleaner_based_on_age(
                &dir_path,
                cutoff_date_str,
                true, // dry run mode
                &mut report,
                &[],
            );

            assert!(result.is_ok());
            assert!(file_path.exists()); // file should not be deleted
        }
    }
}
