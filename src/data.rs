use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Input {
    pub fmt: Fmt,
    pub charts: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Fmt {
    Day,
    Number,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Chart {
    pub title: String,
    pub range: String,
}

pub struct DayInput {
    pub title: String,
    pub start: NaiveDate,
    pub finish: NaiveDate,
}

pub struct NumberInput {
    pub title: String,
    pub start: i64,
    pub finish: i64,
}
