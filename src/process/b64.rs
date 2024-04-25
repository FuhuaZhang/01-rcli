use std::{fs, io::Read};

use anyhow::{Error, Ok};
use base64::{engine::general_purpose, Engine};

use crate::opts::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(&buf),
        Base64Format::Urlsafe => general_purpose::URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(buf)?,
        Base64Format::Urlsafe => general_purpose::URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{:?}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>, Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(fs::File::open(input)?)
    };

    Ok(reader)
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