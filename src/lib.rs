mod cli;
mod process;

pub use process::{get_reader, process_csv, process_decode, process_encode, process_genpass};

pub use cli::{Base64Format, Base64SubCommand, Opts, OutputFormat, SubCommand};
