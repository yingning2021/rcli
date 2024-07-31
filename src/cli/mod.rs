use std::path::Path;

use self::csv::CsvOptions;
use clap::Parser;
use genpass::GenPassOpts;

mod base64;
mod csv;
mod genpass;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli", version ,author, about, long_about= None,)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV and")]
    Csv(CsvOptions),

    #[command(name = "genpass", about = "Generator random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Generator random password")]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)] // 在正式的版本中不编译它
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
