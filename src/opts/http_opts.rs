use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    pub path: PathBuf,

    pub port: u16,
}
