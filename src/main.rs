use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };

            process_csv(&csv_opts.input, &output, csv_opts.format)?;
        }
        SubCommand::Genpass(genpass_opts) => {
            let password = process_genpass(
                genpass_opts.length,
                genpass_opts.uppercase,
                genpass_opts.lowercase,
                genpass_opts.number,
                genpass_opts.symbol,
            );
            println!("{}", password);
        }
        SubCommand::Base64(base64_opts) => match base64_opts {
            Base64SubCommand::Encode(encode_opts) => {
                process_encode(&encode_opts.input, encode_opts.format)?;
            }
            Base64SubCommand::Decode(decode_opts) => {
                process_decode(&decode_opts.input, decode_opts.format)?;
            }
        },
    }
    Ok(())
}
