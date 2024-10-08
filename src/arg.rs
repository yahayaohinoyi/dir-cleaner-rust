use clap::{Arg, ArgGroup, Command};

const APP: &str = "Directory cleaner";

#[derive(Debug)]
pub struct Args {
    pub types: Vec<String>,
    pub min_size: Option<u64>,
    pub dir: String,
    pub dry_run: bool,
    pub remove_duplicates: bool,
    pub age: Option<String>,
    pub files_to_ignore: Vec<String>,
    pub config_file: Option<String>,
}

impl Args {
    pub fn clear(&mut self) {
        self.min_size = None;
        self.types = vec![];
        self.dir = "".to_string();
        self.age = None;
        self.dry_run = false;
        self.files_to_ignore = vec![];
        self.remove_duplicates = false;
    }
}

pub fn parse_args() -> Args {
    let arg = Command::new(APP)
        .version("1.0")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("dir")
                .value_name("DIRECTORY")
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
        .arg(
            Arg::new("age")
                .short('a')
                .long("age")
                .required(false)
                .value_parser(clap::value_parser!(String))
                .help("Specify the cutoff date in YYYY-MM-DD format"),
        )
        .arg(
            Arg::new("ignore_paths")
                .short('i')
                .long("files_to_ignore")
                .required(false)
                .num_args(1..) // Allow multiple values
                .help("Files to ignore (space-separated)"),
        )
        .arg(
            Arg::new("config_file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Config file containing cleanup metadata"),
        )
        .group(
            ArgGroup::new("directrory_or_config_file")
                .args(&["directory", "config_file"])
                .required(true), // One of these must be present
        )
        .get_matches();

    let dir = match arg.try_get_one::<String>("directory") {
        Ok(Some(dir)) => dir.to_string(),
        Ok(None) => {
            println!("No directory provided, expecting config file");
            String::new()
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

    let age: Option<String> = match arg.try_get_one::<String>("age") {
        Ok(Some(date)) => Some(date.to_string()),
        Ok(None) => None,
        Err(_) => None,
    };

    let files_to_ignore = match arg.get_many::<String>("ignore_paths") {
        Some(types) => types.cloned().collect(),
        None => Vec::new(),
    };

    let config_file: Option<String> = match arg.try_get_one::<String>("config_file") {
        Ok(Some(val)) => Some(val.to_string()),
        Ok(None) => None,
        Err(_) => None,
    };

    Args {
        types,
        min_size,
        dir,
        dry_run,
        remove_duplicates,
        age,
        files_to_ignore,
        config_file,
    }
}
