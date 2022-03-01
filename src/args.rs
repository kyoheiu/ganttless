use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author = "kyoheiu", version = "0.1.0", about = "less bloated gantt chart", long_about = None)]
pub struct Args {
    pub input_file: Option<String>,

    /// Show simple chart
    #[clap(short, long)]
    pub simple: bool,

    /// Use day format
    #[clap(short, long)]
    pub day: bool,

    /// Use number format
    #[clap(short, long)]
    pub number: bool,

    /// input data. <title>:<range>
    #[clap(short, long, takes_value(true), multiple_values(true))]
    pub input: Option<Vec<String>>,
}
