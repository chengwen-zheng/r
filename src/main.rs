use clap::Parser;
use rcli::{CmdExector, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    tracing_subscriber::fmt::init();
    println!("{:?}", opts);

    opts.cmd.execute().await?;

    Ok(())
}
