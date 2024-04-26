use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};
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
    }
    Ok(())
}
