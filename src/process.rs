use anyhow::Ok;
use serde::{Deserialize, Serialize};
use crate::opts::CsvOpts;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
    #[serde(rename = "DOB")]
    dob: String,
}

pub fn process_csv(csv_opts: CsvOpts) -> anyhow::Result<()> {
  let mut rdr = csv::Reader::from_path(csv_opts.input)?;
  let mut ret = Vec::with_capacity(128);
  for result in rdr.deserialize() {
      let player: Player = result?;
      ret.push(player);
  }
  let json = serde_json::to_string_pretty(&ret)?;
  std::fs::write(csv_opts.output, json)?;
  Ok(())
}
