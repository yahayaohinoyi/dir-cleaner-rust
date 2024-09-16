use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn directory_cleaner_based_on_file_type(
    dir: &String,
    paths_to_clear: &[String],
    dry_run: bool,
) -> Result<()> {
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
                fs::metadata(path)
                    .with_context(|| format!("Failed to read metadata for file: {:?}", path))?;

                println!("Deleting file: {:?}", path);

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

    Ok(())
}
