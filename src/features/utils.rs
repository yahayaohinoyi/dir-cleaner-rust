use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use colored::*;
use std::cmp::Ordering;
use std::fs::{self};

use crate::ReportData;

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
    del_meta: (u32, u32),
) {
    report_data.files_deleted += del_meta.0;
    report_data.files_scanned += 1;
    report_data.total_file_size_deleted += del_meta.1;
    report_data.total_files_retained = report_data.files_scanned - del_meta.0;

    report_data.total_file_size_retained += metadata.len() as u32;
    report_data.total_file_size_retained -= del_meta.1;

    if let Some(pth) = path.to_str() {
        match del_meta.0.cmp(&0) {
            Ordering::Equal => {
                report_data.paths_retained.push(pth.to_string());
            }
            Ordering::Greater => {
                report_data.paths_deleted.push(pth.to_string());
            }
            Ordering::Less => {}
        }
    }
}
