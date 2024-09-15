use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn directory_cleaner(dir: &String, paths_to_clear: &[String], size: u64) -> Result<()> {
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

            if paths_to_clear.iter().any(|p| ext == p) {
                let metadata = fs::metadata(path)
                    .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                if metadata.len() >= size {
                    println!("Deleting file: {:?}", path);
                    fs::remove_file(path)
                        .with_context(|| format!("Failed to delete file: {:?}", path))?;
                }
            }
        } else {
            println!("File does not exist, {}", path.display())
        }
    }

    Ok(())
}
