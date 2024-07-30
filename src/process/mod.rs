mod b64;
mod csv_convert;
mod gen_pass;

use anyhow::Result;
use std::{fs::File, io::Read};

pub use b64::{process_decode, process_encode};
pub use csv_convert::process_csv;
pub use gen_pass::process_genpass;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
