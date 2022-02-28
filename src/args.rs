use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author = "kyoheiu", version = "0.1.0", about = "less bloated gantt chart", long_about = None)]
pub struct Args {
    pub input_file: Option<String>,

    /// Show simple chart
    #[clap(short, long)]
    pub simple: bool,
}
