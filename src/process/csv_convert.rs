use anyhow::Result;
use serde_json::Value;
use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::cli::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers().cloned()?;
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?, // TODO toml Error: unsupported rust type
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(format!("{}.{}", output, format), content)?;
    Ok(())
}
