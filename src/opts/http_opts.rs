use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

use crate::{process_http_serve, CmdExecutor};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
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

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.path, self.port).await
    }
}
