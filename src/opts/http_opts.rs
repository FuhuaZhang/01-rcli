use clap::Parser;
use std::path::PathBuf;

use crate::{process_http_serve, CmdExecutor};

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(long)]
    pub path: PathBuf,

    #[arg(long)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.path, self.port).await
    }
}
