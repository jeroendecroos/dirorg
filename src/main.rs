use clap::Parser;
use log::{info, debug};
use std::fs;
use std::io;
use std::path::PathBuf;
use text_io::read;
use std::process;


// Need to change this to Enum clap(arg_enum)
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

    /// Don't perform any action
    #[clap(short, long)]
    dryrun: bool,

    /// Iterate through files in this order
    #[clap(long, possible_values(ORDER_MODES), default_value = ORDER_MODES[0])]
    order: String,

}

fn get_directory_content(indir: PathBuf, _order:String) -> Result<Vec<PathBuf>, std::io::Error>{
    let mut entries = fs::read_dir(indir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    entries.sort();
    Ok(entries)
}

fn get_file_destination(filepath: PathBuf, outdir: PathBuf) -> (PathBuf, PathBuf) {
    let mut destination_subdirs = get_directory_content(outdir.clone(), String::from(ORDER_MODES[0])).unwrap();
    destination_subdirs.insert(0, PathBuf::from("."));
    let amount = destination_subdirs.len();
    println!("The following subdirs exist:");
    let selection = loop {
        for (i, dirpath) in destination_subdirs.iter().enumerate() {
            println!("    {}: {}", i, dirpath.display());
        }
        println!("which subdir?");
        let destination : usize = read!();
        if destination < amount {
            break destination
        } else {
            println!("None existing number, try again")
        }
    };
    let selected_subdir = &destination_subdirs[selection];
    let mut destination_dir = outdir.clone();
    destination_dir.push(selected_subdir);
    info!("selected {}", destination_dir.display());
    (filepath.clone(), destination_dir)
}

fn move_file(original: PathBuf, destination: PathBuf, dryrun: bool) {
    info!("Copying {} to {}", original.display(), destination.display());
    if !dryrun {
       let srcdir = fs::canonicalize(&original).unwrap() ;
       let dstdir = fs::canonicalize(&destination).unwrap() ;
       debug!("Full path is {} to {}", srcdir.display(), dstdir.display());
       fs::copy(srcdir,  dstdir).unwrap(); 
    }
    info!("Copying {} to {}", original.display(), destination.display());
}


fn process_filepaths(filepaths: Vec<PathBuf>, outdir: PathBuf, dryrun: bool) {
    for filepath in filepaths {
        info!("Processing {}", filepath.display());
        let (filepath, destination) = get_file_destination(filepath, outdir.clone());
        move_file(filepath, destination, dryrun);
    }
}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    info!("Will organize {}", args.indir);
    info!("Into directory {}", args.outdir);
    info!("iterating {}", args.order);
    let outdir = PathBuf::from(args.outdir);
    let indir = PathBuf::from(args.indir);
    if ! outdir.exists() {
        eprintln!("output dir does not exist, provide an existing path");
        process::exit(1)
    }
    if ! indir.exists() {
        eprintln!("input dir does not exist, provide an existing path");
        process::exit(1)
    }
    let filepaths = get_directory_content(indir, args.order).unwrap();
    process_filepaths(filepaths, outdir, args.dryrun);
}

