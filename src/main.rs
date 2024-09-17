use anyhow::Result;
use colored::*;

mod arg;
mod features;

fn main() -> Result<()> {
    let args = arg::parse_args();

    // precedence: file types, then file size. In case they both show up together in the command
    // we'd do a refactor on here when we consider chaining

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
        )?;
    }
    if let Some(val) = args.min_size {
        println!("Cleaning directory based on min size: {:?}", args.dir);
        println!("Minimum file size: {} bytes", val);
        features::cleaner_file_size::directory_cleaner_based_on_file_size(
            &args.dir,
            val,
            args.dry_run,
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
        )?;
    }
    if let Some(age_value) = args.age {
        println!(
            "Cleaning directory based on age: {:?}",
            age_value
        );
        features::cleaner_last_modified_time::directory_cleaner_based_on_age(
            &args.dir,
            age_value,
            args.dry_run
        )?;
    }

    println!("Cleaning completed successfully.");

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

        let file_types = vec!["txt".to_string()];
        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_type::directory_cleaner_based_on_file_type(
            &dir_str,
            &file_types,
            false,
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

        let file_types = vec!["txt".to_string()];
        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_type::directory_cleaner_based_on_file_type(
            &dir_str,
            &file_types,
            true,
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

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_size::directory_cleaner_based_on_file_size(&dir_str, 2000, false)?;

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

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_size::directory_cleaner_based_on_file_size(&dir_str, 2000, true)?;

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

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str, false,
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

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str, true,
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

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str, false,
        )?;

        assert!(
            !(file_path_1.exists() && file_path_2.exists()),
            "One of test 1 or test 2 should be deleted"
        );

        features::cleaner_file_size::directory_cleaner_based_on_file_size(&dir_str, 4500, false)?;

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

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_duplicate::directory_cleaner_based_on_duplicate_files(
            &dir_str, true,
        )?;

        assert!(
            file_path_1.exists() && file_path_2.exists() && file_path_3.exists(),
            "All files should still exist"
        );

        features::cleaner_file_size::directory_cleaner_based_on_file_size(&dir_str, 4500, true)?;

        assert!(
            file_path_1.exists() && file_path_2.exists() && file_path_3.exists(),
            "All files should still exist"
        );
        Ok(())
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;
        use tempfile::tempdir;
        use chrono::{Utc, TimeZone, DateTime};
        use filetime::{set_file_times, FileTime};
        use std::path::Path;

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

            // Run directory cleaner
            let result = features::cleaner_last_modified_time::directory_cleaner_based_on_age(
                &dir_path,
                cutoff_date_str,
                false, // not a dry run, actually delete files
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

            // Run directory cleaner in dry run mode
            let result = features::cleaner_last_modified_time::directory_cleaner_based_on_age(
                &dir_path,
                cutoff_date_str,
                true, // dry run mode
            );

            assert!(result.is_ok());
            assert!(file_path.exists()); // file should not be deleted
        }
    }

}
