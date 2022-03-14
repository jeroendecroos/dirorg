use clap::Parser;
use log::info;
use std::fs;


const ORDER_MODES: &[&str] = &["alphabetic"];

/// Every needed to clean out a directory in which you blindly stored all kind of files?
/// Use this to rganize files in a directory in to some structure
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    /// the directory to organize
    #[clap(short, long)]
    indir: String,

    /// Where to put the organized files
    #[clap(short, long)]
    outdir: String,

    /// Iterate through files in this order
    #[clap(long, possible_values(ORDER_MODES), default_value = ORDER_MODES[0])]
    order: String,

}


fn main() {
    env_logger::init();
    let args = Cli::parse();
    info!("Will organize {}", args.indir);
    info!("Into directory {}", args.outdir);
    info!("iterating {}", args.order);
    let directories = fs::read_dir(args.indir);
}

