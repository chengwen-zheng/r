use crate::{process_genpass, CmdExector};
use clap::{arg, Parser};
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbols,
        )?;

        let estimate = zxcvbn(&ret, &[]).unwrap();
        println!("Password: {}", ret);
        println!("Score: {:?}", estimate.score());
        Ok(())
    }
}
