use std::str::FromStr;

use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::{process_decode, process_encode, CmdExecutor};

use super::verify_file;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode")]
    Base64Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode")]
    Base64Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser=parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> Result<()> {
        process_encode(&self.input, self.format)?;
        Ok(())
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        println!("{:?}", String::from_utf8(decoded));
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
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
