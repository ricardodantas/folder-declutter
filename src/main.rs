use std::path::Path;
use std::time::{Duration, SystemTime};

use clap::Parser;
mod process_dir;
use process_dir::process_dir;


// Define CLI arguments structure using clap
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, short)]
    /// Folder path to search files in
    folder: String,

    #[arg(long, short)]
    /// Number of days (files older than this will be deleted)
    older_than_days: u64,
}

fn main() {

    let args = Args::parse();

    let cutoff_duration = Duration::from_secs(args.older_than_days * 24 * 3600);
    let cutoff = SystemTime::now() - cutoff_duration;

    println!("\n🔎 Starting to declutter files from \"{}\"...", args.folder);

    process_dir(Path::new(&args.folder), cutoff);
}
