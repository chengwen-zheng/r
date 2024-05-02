use crate::cli::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use std::io::Read;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> anyhow::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    Ok(String::from_utf8(decoded)?)
}

mod tests {
    #[allow(unused_imports)] // for cargo test to pass]
    use crate::get_reader;

    #[allow(unused_imports)] // for cargo test to pass]
    use super::*;
    #[allow(unused_imports)] // for cargo test to pass]
    use std::io::Cursor;

    #[test]
    fn test_process_encode() {
        let input = "hello world";
        let mut reader = Cursor::new(input);
        let encoded = process_encode(&mut reader, Base64Format::Standard).unwrap();
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let mut reader = get_reader(input).unwrap();
        let decoded = process_decode(&mut reader, Base64Format::UrlSafe).unwrap();
        println!("{}", decoded);
    }
}
