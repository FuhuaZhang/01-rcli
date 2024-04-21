// rcli csv -i input.csv -o output.json --header -d ','

use clap::{arg, Parser, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[command(name = "csv", about = "Show SCV or convert CSV to other format")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(long, default_value_t = true)]
    header: bool,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(csv_opts) => {
            let mut reader = Reader::from_path(csv_opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result?;
                ret.push(record);
            }

            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(csv_opts.output, json)?;
        }
    }

    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
