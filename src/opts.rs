use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(file: &str) -> Result<String, &'static str> {
    if Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err("Input file does not exist")
    }
}
