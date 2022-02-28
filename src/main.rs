use ganttless::{ganttless, MyError};

mod data;
mod ganttless;
mod parse;

fn main() -> Result<(), MyError> {
    let sample = std::fs::read_to_string("sample.yaml")?;
    let result = ganttless(sample)?;
    println!("{}", result.verbose);
    println!("{}", result.simple);
    Ok(())
}
