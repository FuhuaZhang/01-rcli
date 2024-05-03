use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_generate, process_genpass, process_sign,
    process_verify, Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};
use std::fs;

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
            )?;
            println!("{}", password);
        }
        SubCommand::Base64(base64_opts) => match base64_opts {
            Base64SubCommand::Encode(encode_opts) => {
                process_encode(&encode_opts.input, encode_opts.format)?;
            }
            Base64SubCommand::Decode(decode_opts) => {
                let decoded = process_decode(&decode_opts.input, decode_opts.format)?;
                println!("{:?}", String::from_utf8(decoded));
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(sign_opts) => {
                let signed = process_sign(&sign_opts.input, &sign_opts.key, sign_opts.format)?;
                println!("{:?}", signed);
            }
            TextSubCommand::Verify(verify_opts) => {
                let result = process_verify(
                    &verify_opts.input,
                    &verify_opts.key,
                    verify_opts.format,
                    verify_opts.signature.as_bytes(),
                )?;
                println!("{:?}", result);
            }
            TextSubCommand::Generate(generate_opts) => {
                let key = process_generate(&generate_opts.format)?;
                match generate_opts.format {
                    TextSignFormat::Blake3 => {
                        let name = generate_opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = generate_opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
