use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>/?";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbols: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::with_capacity(length as usize);
    let mut charset = Vec::new();
    if uppercase {
        charset.extend_from_slice(UPPER);
        pass.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lowercase {
        charset.extend_from_slice(LOWER);
        pass.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        charset.extend_from_slice(NUMBER);
        pass.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbols {
        charset.extend_from_slice(SYMBOLS);
        pass.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS won't be empty"));
    }
    for _ in 0..(length - pass.len() as u8) {
        pass.push(*charset.choose(&mut rng).expect("charset won't be empty"));
    }

    pass.shuffle(&mut rng);

    Ok(String::from_utf8(pass).unwrap())
}
