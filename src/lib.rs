mod process;
pub use process::{process_csv, process_decode, process_encode, process_genpass};

mod opts;
pub use opts::{Base64SubCommand, CsvOpts, GenpassOpts, Opts, OutputFormat, SubCommand};
