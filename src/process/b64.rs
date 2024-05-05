use base64::{engine::general_purpose, Engine};
use std::io::Read;

use crate::{cli::Base64Format, get_reader};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(&buf),
        Base64Format::Urlsafe => general_purpose::URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", encoded);
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(buf)?,
        Base64Format::Urlsafe => general_purpose::URL_SAFE_NO_PAD.decode(buf)?,
    };
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "-";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/tmp.b64";
        let format = Base64Format::Urlsafe;
        assert!(process_decode(input, format).is_ok());
    }
}
