use crate::ReportData;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use colored::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::{self};
use std::path::PathBuf;

pub fn delete_file(
    path: &std::path::Path,
    dry_run: bool,
    files_to_ignore: &HashSet<PathBuf>,
) -> Result<()> {
    if let Some(file_name) = path.file_name() {
        if files_to_ignore
            .iter()
            .any(|ignore_path| ignore_path.file_name() == Some(file_name))
        {
            println!("Skipping ignored file: {:?}", path);
            return Ok(()); // Skip this file if its name matches any in the ignore set
        }
    }
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

    // Create the NaiveTime for midnight
    let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    // Combine the NaiveDate and NaiveTime to create a NaiveDateTime
    let naive_datetime = naive_date.and_time(naive_time);

    // Convert the NaiveDateTime to a DateTime<Utc>
    let cutoff_date = Utc.from_utc_datetime(&naive_datetime);

    Ok(cutoff_date)
}

pub fn collect_metrics(
    report_data: &mut ReportData,
    metadata: std::fs::Metadata,
    path: &std::path::Path,
    del_meta: (u32, u64),
) {
    report_data.files_deleted += del_meta.0;
    report_data.files_scanned += 1;
    report_data.total_file_size_deleted += del_meta.1;
    report_data.total_files_retained = report_data.files_scanned - report_data.files_deleted;

    report_data.total_file_size_retained += metadata.len();
    report_data.total_file_size_retained -= del_meta.1;

    if let Some(pth) = path.to_str() {
        match del_meta.0.cmp(&0) {
            Ordering::Equal => {
                report_data.paths_retained.push(pth.to_string());
            }
            Ordering::Greater => {
                report_data.paths_deleted.push(pth.to_string());
            }
            _ => {}
        }
    }
}
