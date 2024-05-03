use std::str::FromStr;

use anyhow::{Error, Result};
use clap::{Parser, Subcommand};

use crate::{process_decode, process_encode, CmdExecutor};

use super::verify_file;

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "decode")]
    Decode(DecodeOpts),
}

impl CmdExecutor for Base64SubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            Base64SubCommand::Encode(encode_opts) => encode_opts.execute().await,
            Base64SubCommand::Decode(decode_opts) => decode_opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser=parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for EncodeOpts {
    async fn execute(self) -> Result<()> {
        process_encode(&self.input, self.format)?;
        Ok(())
    }
}

impl CmdExecutor for DecodeOpts {
    async fn execute(self) -> Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        println!("{:?}", String::from_utf8(decoded));
        Ok(())
    }
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
