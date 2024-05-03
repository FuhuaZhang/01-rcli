mod process;
pub use process::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_http_serve, process_sign, process_verify,
};

mod opts;
pub use opts::{
    Base64SubCommand, CsvOpts, GenpassOpts, HttpSubCommand, Opts, OutputFormat, SubCommand,
    TextSignFormat, TextSubCommand,
};

mod utils;
pub use utils::get_reader;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
