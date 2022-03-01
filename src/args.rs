use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author = "kyoheiu <kyoheiu@outlook.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "less bloated gantt chart")]
#[clap(
    long_about = "This app produces a gantt chart, less bloated, in ASCII. For more detail, check https://github.com/kyoheiu/ganttless"
)]
pub struct Args {
    pub input_file: Option<String>,

    /// Print simple chart
    #[clap(short, long)]
    pub simple: bool,

    /// Use day format
    #[clap(short, long)]
    pub day: bool,

    /// Use number format
    #[clap(short, long)]
    pub number: bool,

    /// Input data as args: title=yyyy-m-d:yyyy-m-d or title=integer:integer
    #[clap(short, long, takes_value(true), multiple_values(true))]
    pub input: Option<Vec<String>>,
}
