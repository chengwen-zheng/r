mod base64;
mod csv;
mod genpass;
mod text;
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
