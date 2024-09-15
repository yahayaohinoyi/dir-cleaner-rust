use clap::{Arg, Command};

const APP: &str = "Directory cleaner";
pub struct Args {
    pub types: Vec<String>,
    pub min_size: Option<u64>,
    pub dir: String,
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

    Args {
        types,
        min_size,
        dir,
    }
}
