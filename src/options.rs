use crate::errors::{Error, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum OptionValue {
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
pub struct Options {
    pub open: bool,
    pub prefix: Option<String>,
    pub extension: String,
    pub editor: String,
}

impl Options {
    pub fn new() -> Self {
        Self {
            open: false,
            prefix: None,
            extension: "md".to_string(),
            editor: "vim".to_string(),
        }
    }
}

fn parse_option_value(s: &str) -> Result<OptionValue> {
    match s {
        "true" => Ok(OptionValue::Bool(true)),
        "false" => Ok(OptionValue::Bool(false)),
        _ if s.len() >= 2
            && s.chars().nth(0).unwrap() == '"'
            && s.chars().last().unwrap() == '"' =>
        {
            Ok(OptionValue::Str(s[1..(s.len() - 1)].to_string()))
        }
        _ => Err(Error::OptionParsingFailed("".to_string())),
    }
}

fn parse_option(s: &str) -> Result<(&str, OptionValue)> {
    let splitted: Vec<&str> = s.split("=").collect();
    let (key, value) = (splitted[0].trim(), splitted[1].trim());
    Ok((key, parse_option_value(value)?))
}

pub fn parse(filename: &str) -> Result<Options> {
    let mut options = Options::new();

    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let line = line.unwrap();
        match parse_option(line.as_str())? {
            ("open", OptionValue::Bool(b)) => options.open = b,
            ("prefix", OptionValue::Str(p)) => options.prefix = Some(p),
            ("extension", OptionValue::Str(s)) => options.extension = s,
            ("editor", OptionValue::Str(s)) => options.editor = s,
            (key, _) => {
                return Err(Error::OptionParsingFailed(format!(
                    "found unknown key: {}",
                    key
                )))
            }
        }
    }

    Ok(options)
}
