use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
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

pub fn parse_cutoff_date(date_str: &str) -> anyhow::Result<DateTime<Utc>> {
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .with_context(|| format!("Invalid date format: {}", date_str))?;
    let cutoff_date = Utc.from_utc_date(&naive_date).and_hms(0, 0, 0);
    Ok(cutoff_date)
}
