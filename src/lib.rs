mod process;
use enum_dispatch::enum_dispatch;
pub use process::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_http_serve, process_sign, process_verify,
};

mod cli;
pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand, CsvOpts, GenerateKeyOpts, GenpassOpts,
    HttpServeOpts, HttpSubCommand, Opts, OutputFormat, SubCommand, TextSignFormat, TextSignOpts,
    TextSubCommand, TextVerifyOpts,
};

mod utils;
pub use utils::get_reader;

#[enum_dispatch]
#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
