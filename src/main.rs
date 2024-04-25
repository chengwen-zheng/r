use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// rcli csv -i data.csv -o data.json --header -d ','
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            println!("{:?}", csv_opts);
            process_csv(csv_opts)?;
        }
    }
    Ok(())
}
