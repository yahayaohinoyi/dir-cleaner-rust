use anyhow::Result;

mod arg;
mod features;

fn main() -> Result<()> {
    let args = arg::parse_args();

    println!("Cleaning directory: {:?}", args.dir);
    println!("File types to clean: {:?}", args.types);
    println!("Minimum file size: {} bytes", args.min_size);

    features::cleaner::directory_cleaner(&args.dir, &args.types, args.min_size)?;

    println!("Cleaning completed successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_directory_cleaner() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.txt");
        let file = File::create(&file_path)?;
        file.set_len(1000)?; // 1000 bytes

        let file_types = vec!["txt".to_string()];
        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner::directory_cleaner(&dir_str, &file_types, 500)?;

        assert!(!file_path.exists(), "File should have been deleted");

        Ok(())
    }

    // Uncomment to test size constraint
    // #[test]
    // fn test_directory_cleaner_should_not_delete_due_to_smaller_size() -> Result<()> {
    //     let temp_dir = tempdir()?;
    //     let file_path = temp_dir.path().join("test.txt");
    //     let file = File::create(&file_path)?;
    //     file.set_len(1000)?; // 1000 bytes
    //
    //     let file_types = vec!["txt".to_string()];
    //     let dir_str = temp_dir.path().to_str().unwrap().to_string();
    //     features::cleaner::directory_cleaner(&dir_str, &file_types, 2000)?;
    //
    //     assert!(
    //         file_path.exists(),
    //         "File should not have been deleted due to size constraint"
    //     );
    //
    //     // temp_dir will be cleaned up here when it goes out of scope
    //
    //     Ok(())
    // }
}
