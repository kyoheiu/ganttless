mod args;
mod data;
mod ganttless;
mod parse;

use args::*;
use clap::Parser;
use data::*;
use ganttless::{ganttless, MyError};
use parse::input_to_tuple;

fn main() -> Result<(), MyError> {
    let args = Args::parse();
    match args.input {
        Some(input) => {
            let mut map = std::collections::HashMap::new();
            for input in input {
                let input = input_to_tuple(input)?;
                map.insert(input.0, input.1);
            }
            if args.day {
                let input = Input {
                    fmt: Fmt::Day,
                    charts: map,
                };
                let result = ganttless(input)?;
                if args.simple {
                    println!("{}", result.simple);
                } else {
                    println!("{}", result.verbose);
                }
            } else if args.number {
                let input = Input {
                    fmt: Fmt::Number,
                    charts: map,
                };
                let result = ganttless(input)?;
                if args.simple {
                    println!("{}", result.simple);
                } else {
                    println!("{}", result.verbose);
                }
            } else {
                println!("You need to specify format: Day(-d) or Number(-n)");
            }
        }
        None => match args.input_file {
            Some(path) => {
                let sample = std::fs::read_to_string(path)?;
                let de: Input = serde_yaml::from_str(&sample)?;
                let result = ganttless(de)?;
                if args.simple {
                    println!("{}", result.simple);
                } else {
                    println!("{}", result.verbose);
                }
            }
            None => {
                let mut command = clap::Command::new("gnt");
                let _ = command.print_long_help();
            }
        },
    }
    Ok(())
}
