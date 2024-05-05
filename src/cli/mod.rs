mod base64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show SCV or convert CSV to other format")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),

    #[command(subcommand, about = "encode & decode with base64")]
    Base64(Base64SubCommand),

    #[command(subcommand, about = "sign & verify a text")]
    Text(TextSubCommand),

    #[command(subcommand, about = "serve static files")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
