use core::fmt;
use std::{path::Path, str::FromStr};

use clap::Parser;

use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short, long,value_parser=verify_input_file )]
    pub input: String,

    #[arg(long,value_parser=parse_format,default_value="json")]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat> {
    format.parse()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => anyhow::bail!("Invalid format {}", s),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
