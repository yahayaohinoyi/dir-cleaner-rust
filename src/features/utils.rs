use anyhow::{Context, Ok, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use colored::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::{self};
use std::io::BufRead;
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
    report_data: &mut crate::ReportData,
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

pub fn build_args(args: &mut crate::arg::Args, line_arg: &Vec<&str>) -> Result<()> {
    if line_arg.len() <= 0 {
        return Ok(());
    }

    let verb = line_arg[0];
    match verb {
        "--dir" | "-d" => {
            // for duplicate directory in file
            if args.dir.len() > 0 {
                eprint!("Found duplicate verb, {}", verb);
                return Ok(());
            }

            if line_arg.len() > 1 {
                args.dir = line_arg[1].to_string();
            } else {
                eprint!(
                    "Expected at least one arg for specified verb, {}, skipping...",
                    verb
                );
            }
        }
        "--size" | "-s" => {
            if args.min_size.is_some() {
                eprint!("Found duplicate verb, {}", verb);
                return Ok(());
            }

            if line_arg.len() > 1 {
                let val: u64 = line_arg[1].parse()?;
                args.min_size = Some(val);
            } else {
                eprint!(
                    "Expected at least one arg for specified verb, {}, skipping...",
                    verb
                );
            }
        }
        "--dryrun" | "-n" => {
            if args.dry_run {
                eprint!("Found duplicate verb, {}", verb);
                return Ok(());
            }
            args.dry_run = true;
        }
        "--dedup" | "-r" => {
            if args.remove_duplicates {
                eprint!("Found duplicate verb, {}", verb);
                return Ok(());
            }
            args.remove_duplicates = true;
        }
        "--age" | "-a" => {
            if args.age.is_some() {
                eprint!("Found duplicate verb, {}", verb);
                return Ok(());
            }
            if line_arg.len() > 1 {
                args.age = Some(line_arg[1].to_string());
            } else {
                eprint!(
                    "Expected at least one arg for specified verb, {}, skipping...",
                    verb
                );
            }
        }
        "--files_to_ignore" | "-i" => {
            if args.files_to_ignore.len() > 0 {
                eprint!("Found duplicate verb, {}", verb);
                return Ok(());
            }

            if line_arg.len() > 1 {
                for val in line_arg[1..].iter() {
                    args.files_to_ignore.push(val.to_string());
                }
            } else {
                eprint!(
                    "Expected at least one arg for specified verb, {}, skipping...",
                    verb
                );
            }
        }
        "--types" | "-t" => {
            if args.types.len() > 0 {
                eprint!("Found duplicate verb, {}", verb);
                return Ok(());
            }

            if line_arg.len() > 1 {
                for val in line_arg[1..].iter() {
                    args.types.push(val.to_string());
                }
            } else {
                eprint!(
                    "Expected at least one arg for specified verb, {}, skipping...",
                    verb
                );
            }
        }
        _ => {
            eprint!("unknown verb, {}, skipping...", verb);
        }
    }

    Ok(())
}

pub fn read_file_and_rebuild_args(args: &mut crate::arg::Args) -> Result<()> {
    // ensure we don't have any values lurking in args. Config files
    // takes precedence
    args.clear();
    let config_file = args.config_file.clone();

    if config_file.is_none() {
        eprint!("Config file needed to process this funtion");
        std::process::exit(1);
    }

    let file = config_file.as_ref().unwrap();
    let f = std::fs::File::open(file)?;

    for line in std::io::BufReader::new(f).lines() {
        if let core::result::Result::Ok(line_val) = line {
            let line_arg: Vec<&str> = line_val.split(' ').collect();
            build_args(args, &line_arg)?;
        }
    }

    if !args.dir.len() > 0 {
        eprint!("dir not present in the config file");
        std::process::exit(1);
    }

    Ok(())
}
