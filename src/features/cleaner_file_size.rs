use anyhow::{Context, Result};
use colored::*;
use std::fs;
use walkdir::WalkDir;

pub fn directory_cleaner_based_on_file_size(
    directory: &String,
    size: u64,
    dry_run: bool,
) -> Result<()> {
    for entry in WalkDir::new(directory).into_iter() {
        match entry {
            Ok(dir) => {
                let path = dir.path();

                if path.is_file() {
                    let metadata = fs::metadata(path)
                        .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                    if metadata.len() >= size {
                        if !dry_run {
                            fs::remove_file(path)
                                .with_context(|| format!("Failed to delete file: {:?}", path))?;
                        } else {
                            if let Some(pth) = path.to_str() {
                                println!("\n {} could have been deleted", pth.bold().yellow());
                            }
                        }
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
