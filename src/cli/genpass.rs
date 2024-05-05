use anyhow::Ok;
use clap::Parser;

use crate::{process_genpass, CmdExecutor};

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenpassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", password);
        Ok(())
    }
}
