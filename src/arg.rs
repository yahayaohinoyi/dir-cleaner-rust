use clap::{Arg, Command};

const APP: &str = "Directory cleaner";
pub struct Args {
    pub types: Vec<String>,
    pub min_size: Option<u64>,
    pub dir: String,
    pub dry_run: bool,
    pub remove_duplicates: bool,
}

pub fn parse_args() -> Args {
    let arg = Command::new(APP)
        .version("1.0")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("dir")
                .required(true)
                .help("Directory to clean up"),
        )
        .arg(
            Arg::new("types")
                .short('t')
                .long("types")
                .required(false)
                .num_args(1..) // Allow multiple values
                .help("Types to clean (space-separated)"),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .required(false)
                .value_parser(clap::value_parser!(u64))
                .help("Minimum size to clear (in bytes)"),
        )
        .arg(
            Arg::new("dry_run")
                .short('n')
                .long("dryrun")
                .required(false)
                .value_parser(clap::value_parser!(bool))
                .help("Dry run"),
        )
        .arg(
            Arg::new("remove_duplicates")
                .short('r')
                .long("dedup")
                .required(false)
                .value_parser(clap::value_parser!(bool))
                .help("Remove Duplicate"),
        )
        .get_matches();

    let dir = match arg.try_get_one::<String>("directory") {
        Ok(Some(dir)) => dir.to_string(),
        Ok(None) => {
            eprintln!("Error: Directory to clean is missing");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error processing directory: {:?}", e);
            std::process::exit(1);
        }
    };

    let types = match arg.get_many::<String>("types") {
        Some(types) => types.cloned().collect(),
        None => Vec::new(),
    };

    let min_size: Option<u64> = match arg.get_one::<u64>("size") {
        Some(size) => Some(*size),
        None => None,
    };

    let dry_run: bool = match arg.get_one::<bool>("dry_run") {
        Some(dr) => *dr,
        None => false,
    };

    let remove_duplicates: bool = match arg.get_one::<bool>("remove_duplicates") {
        Some(dr) => *dr,
        None => false,
    };

    Args {
        types,
        min_size,
        dir,
        dry_run,
        remove_duplicates,
    }
}
