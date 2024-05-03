mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod http_opts;
mod text_opts;

use clap::{Parser, Subcommand};
use std::path::Path;

use crate::CmdExecutor;

pub use self::{
    base64_opts::{Base64Format, Base64SubCommand},
    csv_opts::{CsvOpts, OutputFormat},
    genpass_opts::GenpassOpts,
    http_opts::HttpSubCommand,
    text_opts::{TextSignFormat, TextSubCommand},
};
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Subcommand)]
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

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(csv_opts) => csv_opts.execute().await,
            SubCommand::Genpass(genpass_opts) => genpass_opts.execute().await,
            SubCommand::Base64(base64_sub_command) => base64_sub_command.execute().await,
            SubCommand::Text(text_sub_command) => text_sub_command.execute().await,
            SubCommand::Http(http_sub_command) => http_sub_command.execute().await,
        }
    }
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
