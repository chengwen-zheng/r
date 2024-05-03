use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_key_generate, process_text_sign, process_text_verify, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};
use zxcvbn::zxcvbn;
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            println!("{:?}", csv_opts);
            let output = if let Some(output) = csv_opts.output {
                output
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?;
        }
        SubCommand::GenPass(genpass_opts) => {
            println!("{:?}", genpass_opts);
            let ret = process_genpass(
                genpass_opts.length,
                genpass_opts.uppercase,
                genpass_opts.lowercase,
                genpass_opts.number,
                genpass_opts.symbols,
            )?;

            let estimate = zxcvbn(&ret, &[]).unwrap();
            println!("Password: {}", ret);
            println!("Score: {:?}", estimate.score());
        }
        SubCommand::Base64(subcmd) => {
            println!("{:?}", subcmd);

            match subcmd {
                Base64SubCommand::Encode(encode_opts) => {
                    let mut reader = get_reader(&encode_opts.input)?;
                    let encode = process_encode(&mut reader, encode_opts.format);
                    println!("{:?}", encode);
                }
                Base64SubCommand::Decode(decode_opts) => {
                    println!("{:?}", decode_opts);
                    let mut reader = get_reader(&decode_opts.input)?;
                    let decode = process_decode(&mut reader, decode_opts.format);
                    println!("{:?}", decode);
                }
            }
        }
        SubCommand::Text(text_subcmd) => {
            println!("{:?}", text_subcmd);
            match text_subcmd {
                TextSubCommand::Sign(sign_opts) => {
                    println!("{:?}", sign_opts);
                    let mut reader = get_reader(&sign_opts.input)?;
                    let sign = process_text_sign(&mut reader, &sign_opts.key, sign_opts.format)?;
                    // base64 encode the signature
                    let encoded = URL_SAFE_NO_PAD.encode(sign);
                    println!("{}", encoded);
                }

                TextSubCommand::Verify(verify_opts) => {
                    println!("{:?}", verify_opts);
                    let mut reader = get_reader(&verify_opts.input)?;
                    let verified = process_text_verify(
                        &mut reader,
                        &verify_opts.key,
                        verify_opts.format,
                        &verify_opts.sign,
                    )?;
                    if verified {
                        println!("✓ Signature verified");
                    } else {
                        println!("⚠ Signature not verified");
                    }
                }
                TextSubCommand::Generate(generate_opts) => {
                    let key = process_text_key_generate(generate_opts.format)?;
                    for (k, v) in key {
                        fs::write(generate_opts.output_path.join(k), v)?;
                    }
                }
            }
        }
    }
    Ok(())
}
