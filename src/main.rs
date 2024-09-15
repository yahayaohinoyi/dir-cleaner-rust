use anyhow::Result;

mod arg;
mod features;

fn main() -> Result<()> {
    let args = arg::parse_args();

    // precedence: file types, then file size. In case they both show up together in the command
    // we'd do a refactor on here when we consider chaining
    if args.types.len() > 0 {
        println!("Cleaning directory: {:?}", args.dir);
        println!("File types to clean: {:?}", args.types);
        features::cleaner_file_type::directory_cleaner_based_on_file_type(&args.dir, &args.types)?;
    } else if let Some(val) = args.min_size {
        println!("Cleaning directory: {:?}", args.dir);
        println!("Minimum file size: {} bytes", val);
        features::cleaner_file_size::directory_cleaner_based_on_file_size(&args.dir, val)?;
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
        features::cleaner_file_type::directory_cleaner_based_on_file_type(&dir_str, &file_types)?;

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
        file_1.set_len(4000)?; // 3000 bytes
        file_2.set_len(500)?; // 500 bytes

        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        features::cleaner_file_size::directory_cleaner_based_on_file_size(&dir_str, 2000)?;

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
}
