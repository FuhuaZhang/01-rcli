use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Error};
use clap::Parser;

use crate::{process_csv, CmdExecutor};

use super::verify_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: Option<String>,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(short, long, default_value="json", value_parser=parse_format)]
    pub format: OutputFormat,
}

impl CmdExecutor for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", &self.format)
        };

        process_csv(&self.input, &output, self.format)?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Toml,
    Yaml,
}

fn parse_format(format: &str) -> Result<OutputFormat, Error> {
    // OutputFormat::from_str(format)
    format.parse::<OutputFormat>()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "toml" => Ok(OutputFormat::Toml),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow!("Invalid format")),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Toml => "toml",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
