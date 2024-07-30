use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = Vec::new();

    let mut read: Box<dyn Read> = Box::new(io::stdin());
    read.read_to_end(&mut buffer)?;

    println!("{:?}", String::from_utf8(buffer));
    Ok(())
}
