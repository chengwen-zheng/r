use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// rcli csv -i data.csv -o data.json --header -d ','
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
    }
    Ok(())
}
