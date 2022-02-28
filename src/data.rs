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
