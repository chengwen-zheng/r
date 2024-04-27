use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
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
                    let mut reader = rcli::get_reader(&encode_opts.input)?;
                    let encode = process_encode(&mut reader, encode_opts.format);
                    println!("{:?}", encode);
                }
                Base64SubCommand::Decode(decode_opts) => {
                    println!("{:?}", decode_opts);
                    let mut reader = rcli::get_reader(&decode_opts.input)?;
                    let decode = process_decode(&mut reader, decode_opts.format);
                    println!("{:?}", decode);
                }
            }
        }
    }
    Ok(())
}
