use std::{fs, io::Read, path::Path};

use crate::cli::TextSignFormat;
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> anyhow::Result<bool>;
}

pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}
impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let ret = blake3::keyed_hash(&self.key, &buffer);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> anyhow::Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let signature = self.key.sign(&buffer);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> anyhow::Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let signature = ed25519_dalek::Signature::from_bytes(sign.try_into()?);
        Ok(self.key.verify(&buffer, &signature).is_ok())
    }
}

pub fn process_text_sign(reader: &mut dyn Read, key: &str, format: TextSignFormat) -> Result<()> {
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    println!("Signed: {}", signed);

    Ok(())
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &str,
    format: TextSignFormat,
    sign: &str,
) -> Result<bool> {
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(reader, &sign)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(reader, &sign)?
        }
    };

    Ok(verified)
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let mut reader = Cursor::new("hello world!");
        let mut verify_reader = reader.clone();
        let sign = blake3.sign(&mut reader).unwrap();
        let verified = blake3.verify(&mut verify_reader, &sign).unwrap();
        assert!(verified);
        Ok(())
    }
}
