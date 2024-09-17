use anyhow::{Context, Result};
use colored::*;
use std::fs;

pub fn delete_file(path: &std::path::Path, dry_run: bool) -> Result<()> {
    if !dry_run {
        fs::remove_file(path).with_context(|| format!("Failed to delete file: {:?}", path))?;
    } else {
        if let Some(pth) = path.to_str() {
            println!("\n {} could have been deleted", pth.bold().yellow());
        }
    }

    Ok(())
}
