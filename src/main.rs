use std::fs;
use anyhow::{Context, Result};
use walkdir::WalkDir;

mod arg;

fn main() -> Result<()> {
    let args = arg::parse_args();

    println!("Cleaning directory: {:?}", args.dir);
    println!("File types to clean: {:?}", args.types);
    println!("Minimum file size: {} bytes", args.min_size);

    directory_cleaner(&args.dir, &args.types, args.min_size)?;

    println!("Cleaning completed successfully.");

    Ok(())
}


fn directory_cleaner(dir: &String, paths_to_clear: &[String], size: u64) -> Result<()> {
    // skip files that cannot be opened, due to permission issues etc
    for entry in WalkDir::new(dir).into_iter().filter_map(|f| f.ok()) {
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension()
                .and_then(|ex| ex.to_str())
                .unwrap_or("");

            if paths_to_clear.iter().any(|p| ext == p) {
                let metadata = fs::metadata(path)
                    .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                if metadata.len() >= size {
                    println!("Deleting file: {:?}", path);
                    fs::remove_file(path)
                        .with_context(|| format!("Failed to delete file: {:?}", path))?;
                }
            }
        }
    }

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
        directory_cleaner(&dir_str, &file_types, 500)?;

        assert!(!file_path.exists(), "File should have been deleted");

        Ok(())
    }

    #[test]
    fn test_directory_cleaner_should_not_delete_due_to_smaller_size() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.txt");
        let file = File::create(&file_path)?;
        file.set_len(1000)?; // 1000 bytes

        let file_types = vec!["txt".to_string()];
        let dir_str = temp_dir.path().to_str().unwrap().to_string();
        directory_cleaner(&dir_str, &file_types, 2000)?;

        assert!(file_path.exists(), "File should not have been deleted due to size constraint");

        // temp_dir will be cleaned up here when it goes out of scope

        Ok(())
    }
}