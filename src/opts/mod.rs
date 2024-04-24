mod csv_opts;
use clap::{Parser, Subcommand};
pub use csv_opts::{CsvOpts, OutputFormat};

mod genpass_opts;
pub use genpass_opts::GenpassOpts;

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
    // #[command(name = "base64", about = "Base64 encode or decode")]
    // Base64(Base64Opts),
}
