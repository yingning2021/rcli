use std::path::Path;

use clap::{Parser, Subcommand};

use anyhow::Result;

#[derive(Debug, Parser)]
#[command(name = "rcli", version ,author, about, long_about= None,)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV and")]
    Csv(CsvOptions),
}

#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short, long,value_parser=verify_input_file )]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

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
