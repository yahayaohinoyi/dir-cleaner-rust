use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;

pub fn directory_cleaner_based_on_file_size(dir: &String, size: u64) -> Result<()> {
    for entry in WalkDir::new(dir).into_iter() {
        match entry {
            Ok(dir) => {
                let path = dir.path();

                if path.is_file() {
                    let metadata = fs::metadata(path)
                        .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                    if metadata.len() >= size {
                        fs::remove_file(path)
                            .with_context(|| format!("Failed to delete file: {:?}", path))?;
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
