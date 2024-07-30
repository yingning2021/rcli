use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::Base64Format;

use super::get_reader;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    let mut reader = get_reader(input)?;
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    let _data = reader.read_to_string(&mut buf)?;
    // avoid \n
    let buf = buf.trim_end();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // decode出来的数据不一定是string类型的数据
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";

        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_process_decode() -> Result<()> {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        let ret = process_decode(input, format)?;
        println!("{:?}", ret);
        Ok(())
    }
}
