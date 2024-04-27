use crate::cli::OutputFormat;
use anyhow::Ok;
use csv::Reader;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        // 不依赖Player struct 支持所有的csv文件
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    std::fs::write(output, content)?;
    Ok(())
}
