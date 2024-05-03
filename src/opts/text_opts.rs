use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::{process_generate, process_sign, process_verify, CmdExecutor};

use super::verify_file;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with the specified key")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a signed message")]
    Verify(VerifyOpts),
    #[command(name = "genkey", about = "Generate a random key")]
    Generate(GenerateOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser=verify_file)]
    pub key: String,

    #[arg(short, long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for SignOpts {
    async fn execute(self) -> Result<()> {
        process_sign(&self.input, &self.key, self.format)?;
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser=verify_file)]
    pub key: String,

    #[arg(short, long)]
    pub signature: String,

    #[arg(short, long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for VerifyOpts {
    async fn execute(self) -> Result<()> {
        process_verify(
            &self.input,
            &self.key,
            self.format,
            self.signature.as_bytes(),
        )?;
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,

    #[arg(short, long, default_value = "", value_parser=parse_path)]
    pub output: PathBuf,
}

impl CmdExecutor for GenerateOpts {
    async fn execute(self) -> Result<()> {
        let key = process_generate(&self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = self.output;
                fs::write(name.join("ed25519.sk"), &key[0])?;
                fs::write(name.join("ed25519.pk"), &key[1])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid sign format")),
        }
    }
}

fn parse_format(format: &str) -> Result<TextSignFormat, Error> {
    format.parse()
}

fn parse_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(PathBuf::from(path))
    } else {
        Err("Path does not exist or is not a directory")
    }
}
