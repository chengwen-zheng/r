mod base64;
mod csv;
mod genpass;
mod text;
use std::path::{Path, PathBuf};

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::OutputFormat;
pub use self::text::{TextSignFormat, TextSubCommand};
use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cli", version, about, author, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass")]
    GenPass(GenPassOpts),

    #[command(subcommand, about = "Encode/Decode Base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign/Verify text")]
    Text(TextSubCommand),
}

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
