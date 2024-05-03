use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(long)]
    pub path: PathBuf,

    #[arg(long)]
    pub port: u16,
}
