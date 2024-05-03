use std::str::FromStr;

use anyhow::Error;
use clap::{Parser, Subcommand};

use super::verify_file;

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "decode")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser=parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser=parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::Urlsafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, Error> {
    format.parse()
}
