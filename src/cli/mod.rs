mod base64;
mod csv;
mod genpass;
mod http;
mod text;
use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name = "cli", version, about, author, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),

    #[command(subcommand, about = "Encode/Decode Base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign/Verify text")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Serve HTTP")]
    Http(HttpSubCommand),
}

// impl CmdExector for SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::Csv(opts) => opts.execute().await,
//             SubCommand::GenPass(opts) => opts.execute().await,
//             SubCommand::Base64(opts) => opts.execute().await,
//             SubCommand::Text(opts) => opts.execute().await,
//             SubCommand::Http(opts) => opts.execute().await,
//         }
//     }
// }

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

fn verify_file(file: &str) -> Result<String, &'static str> {
    if file == "-" || std::path::Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err("File does not exist")
    }
}
