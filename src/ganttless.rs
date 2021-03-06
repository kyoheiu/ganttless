use super::data::*;
use super::parse::*;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error{"Invalid input data"}]
    InputError,
}

#[derive(Serialize, Debug)]
pub struct ResponseBody {
    pub verbose: String,
    pub simple: String,
}

pub fn ganttless(de: Input) -> Result<ResponseBody, MyError> {
    let mut result_verbose = String::new();
    let mut result_simple;

    let mut input_d_vec: Vec<DayInput> = Vec::new();
    let mut input_n_vec: Vec<NumberInput> = Vec::new();
    for data in de.clone().charts {
        match de.fmt {
            Fmt::Day => {
                let (mut starting_date, mut ending_date) = input_to_date(data.1)?;
                if starting_date > ending_date {
                    std::mem::swap(&mut starting_date, &mut ending_date);
                }
                input_d_vec.push(DayInput {
                    title: data.0,
                    start: starting_date,
                    finish: ending_date,
                });
                input_d_vec.sort_by(|a, b| a.start.cmp(&b.start));
            }
            Fmt::Number => {
                let (mut starting_number, mut ending_number) = input_to_i(data.1)?;
                if starting_number > ending_number {
                    std::mem::swap(&mut starting_number, &mut ending_number);
                }
                input_n_vec.push(NumberInput {
                    title: data.0,
                    start: starting_number,
                    finish: ending_number,
                });
                input_n_vec.sort_by(|a, b| a.start.cmp(&b.start));
            }
        }
    }

    let mut title_max_len = match de.fmt {
        Fmt::Day => input_d_vec.iter().map(|x| x.title.len()).max().unwrap(),
        Fmt::Number => input_n_vec.iter().map(|x| x.title.len()).max().unwrap(),
    };
    if title_max_len < 2 {
        title_max_len = 2;
    }

    match de.fmt {
        Fmt::Day => {
            let iter = input_d_vec.iter();
            let beginning = iter.clone().map(|i| i.start).min().unwrap();
            let ending = iter.map(|i| i.finish).max().unwrap();
            let prefix = " ".repeat(title_max_len);
            let range: usize = (ending - beginning).num_days().try_into()?;
            let range = range + 1;
            let dots = "-".repeat(range);
            let space_needed = beginning.to_string().len() + ending.to_string().len();
            if range > space_needed {
                result_verbose.push_str(&format!(
                    "{prefix} {beginning}{}{ending}",
                    " ".repeat(range - space_needed),
                ));
                result_verbose.push('\n');
                result_verbose.push_str(&format!("{prefix} {dots}"));
                result_verbose.push('\n');
            } else {
                result_verbose.push_str(&format!("{prefix} {beginning} -> {ending}"));
                result_verbose.push('\n');
                result_verbose.push_str(&format!("{prefix} {dots}"));
                result_verbose.push('\n');
            }

            result_simple = result_verbose.clone();

            for e in input_d_vec {
                let title_fmt = " ".repeat(title_max_len - e.title.len());
                let spaces = (e.start - beginning).num_days().try_into()?;
                let pluses: usize = (e.finish - e.start).num_days().try_into()?;
                let pluses = pluses + 1;
                let fills = format!("{}{}", " ".repeat(spaces), "+".repeat(pluses));
                let sub: usize = pluses + 1;
                let beginning_date = e.start.format("%m-%d").to_string();
                let ending_date = e.finish.format("%m-%d").to_string();
                let occupied = beginning_date.len() + ending_date.len() + 4;
                if sub > occupied {
                    result_verbose.push_str(&format!(
                        "{}{title_fmt}|{}{beginning_date} -> {}{ending_date}",
                        e.title,
                        " ".repeat(spaces),
                        " ".repeat(sub - occupied - 1),
                    ));
                    result_verbose.push('\n');
                    result_verbose.push_str(&format!("{}|{fills}", " ".repeat(title_max_len)));
                    result_verbose.push('\n');
                } else {
                    result_verbose.push_str(&format!(
                        "{}{title_fmt}|{}{beginning_date} -> {ending_date}",
                        e.title,
                        " ".repeat(spaces),
                    ));
                    result_verbose.push('\n');
                    result_verbose.push_str(&format!("{}|{fills}", " ".repeat(title_max_len)));
                    result_verbose.push('\n');
                }

                result_simple.push_str(&format!("{}{title_fmt}|{fills}", e.title));
                result_simple.push('\n');
            }
        }
        Fmt::Number => {
            let iter = input_n_vec.iter();
            let beginning = iter.clone().map(|i| i.start).min().unwrap();
            let ending = iter.map(|i| i.finish).max().unwrap();
            let prefix = " ".repeat(title_max_len);
            let range: usize = (ending - beginning + 1).try_into()?;
            let dots = "-".repeat(range);
            let space_needed = beginning.to_string().len() + ending.to_string().len();
            if range > space_needed {
                result_verbose.push_str(&format!(
                    "{prefix} {beginning}{}{ending}",
                    " ".repeat(range - space_needed),
                ));
                result_verbose.push('\n');
                result_verbose.push_str(&format!("{prefix} {dots}"));
                result_verbose.push('\n');
            } else {
                result_verbose.push_str(&format!("{prefix} {beginning} -> {ending}"));
                result_verbose.push('\n');
                result_verbose.push_str(&format!("{prefix} {dots}"));
                result_verbose.push('\n');
            }

            result_simple = result_verbose.clone();

            for e in input_n_vec {
                let title_fmt = " ".repeat(title_max_len - e.title.len());
                let e_begin: usize = (e.start - beginning).try_into()?;
                let e_end: usize = (e.finish - beginning).try_into()?;
                let sub: usize = e_end - e_begin + 1;
                let fills = format!("{}{}", " ".repeat(e_begin), "+".repeat(sub),);
                let occupied = e.start.to_string().len() + e.finish.to_string().len() + 4;
                if sub > occupied {
                    result_verbose.push_str(&format!(
                        "{}{title_fmt}|{}{} -> {}{}",
                        e.title,
                        " ".repeat(e_begin),
                        e.start,
                        " ".repeat(sub - occupied),
                        e.finish
                    ));
                    result_verbose.push_str(&format!("{}|{fills}", " ".repeat(title_max_len)));
                    result_verbose.push('\n');
                } else {
                    result_verbose.push_str(&format!(
                        "{}{title_fmt}|{}{} -> {}",
                        e.title,
                        " ".repeat(e_begin),
                        e.start,
                        e.finish
                    ));
                    result_verbose.push('\n');
                    result_verbose.push_str(&format!("{}|{fills}", " ".repeat(title_max_len)));
                    result_verbose.push('\n');
                }

                result_simple.push_str(&format!("{}{title_fmt}|{fills}", e.title));
                result_simple.push('\n');
            }
        }
    };
    let j: ResponseBody = ResponseBody {
        verbose: result_verbose,
        simple: result_simple,
    };
    Ok(j)
}
