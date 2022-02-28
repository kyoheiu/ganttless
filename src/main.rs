mod args;
mod data;
mod ganttless;
mod parse;

use args::*;
use clap::Parser;
use ganttless::{ganttless, MyError};

fn main() -> Result<(), MyError> {
    let args = Args::parse();
    let sample = std::fs::read_to_string(args.input_file.unwrap())?;
    let result = ganttless(sample)?;
    if args.simple {
        println!("{}", result.simple);
    } else {
        println!("{}", result.verbose);
    }
    Ok(())
}
