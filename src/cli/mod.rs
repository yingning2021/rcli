use self::csv::CsvOptions;
use clap::Parser;
use genpass::GenPassOpts;

mod base64;
mod csv;
mod genpass;

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
    // #[command(subcommand, about = "Generator random password")]
    // Base64(Base64SubCommand),
}
