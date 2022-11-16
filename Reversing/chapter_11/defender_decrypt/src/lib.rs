use std::{error::Error, fs};

use argh::FromArgs;
use sha1_smol::Sha1;

use crate::decrypt::{brute_force, decrypt};

mod decrypt;

#[derive(Debug)]
enum KeyOrSignature {
    Key(u32),
    Signature(Vec<u8>),
}

#[derive(Debug)]
pub(crate) struct Block {
    base: usize,
    length: usize,
    key_or_signature: KeyOrSignature,
}

impl Block {
    fn interpret(v: &toml::Value) -> Option<Self> {
        let base = v.get("base")?.as_integer()? as usize;
        let length = v.get("length")?.as_integer()? as usize;
        let key = v.get("key").and_then(|v| {
            let i = v.as_integer()?;
            Some(i as u32)
        });
        let signature = v.get("signature").and_then(|v| {
            v.as_array()?
                .iter()
                .map(|v| -> Option<u8> {
                    let i = v.as_integer()?;
                    Some(i as u8)
                })
                .collect::<Option<Vec<u8>>>()
        });

        let key_or_signature = match (key, signature) {
            (Some(key), _) => KeyOrSignature::Key(key),
            (_, Some(sig)) => KeyOrSignature::Signature(sig),
            _ => return None,
        };

        Some(Block {
            base,
            length,
            key_or_signature,
        })
    }
}

pub(crate) struct Analysis {
    exec_hash: String,
    code_offset: usize,
    blocks: Vec<Block>,
}

impl Analysis {
    fn parse() -> Result<Self, Box<dyn Error>> {
        const ANALYSIS_PATH: &str = "./analysis.toml";
        let buf = fs::read(ANALYSIS_PATH)?;
        let analysis: toml::Value = toml::from_slice(&buf)?;

        let exec_hash = analysis
            .get("defender_sha1")
            .and_then(|v| {
                let hash = v.as_str()?;
                Some(hash.to_string())
            })
            .ok_or("key 'defender_sha1' not found or malformed, expected string")?;

        let code_offset = analysis
            .get("code_offset")
            .and_then(|v| {
                let offset = v.as_integer()?;
                Some(offset as usize)
            })
            .ok_or("key 'code_offset' not found or malformed, expected integer")?;

        let blocks: Vec<Block> = analysis
            .get("blocks")
            .and_then(|v| {
                v.as_array()?
                    .iter()
                    .map(Block::interpret)
                    .collect::<Option<_>>()
            })
            .ok_or("key 'blocks' not found or malformed, expected array")?;

        Ok(Self {
            exec_hash,
            code_offset,
            blocks,
        })
    }
}

#[derive(FromArgs)]
/// Decrypt encrypted code section in Defender.exe, the resulting executable is not a valid
/// program but useful for reversing purposes.
pub struct Config {
    /// path to Defender.exe
    #[argh(positional)]
    in_path: String,
    /// path to output decrypted executable
    #[argh(positional)]
    out_path: String,
}

pub fn unpack(config: &Config) -> Result<(), Box<dyn Error>> {
    let analysis = Analysis::parse()?;
    let mut buf = fs::read(&config.in_path)?;

    let hash = Sha1::from(&buf).digest().to_string();
    if hash != analysis.exec_hash {
        return Err(
            "Defender.exe doesn't match the recorded hash, executable may be malicious".into(),
        );
    }

    for Block {
        base,
        length,
        key_or_signature,
    } in &analysis.blocks
    {
        let offset = base - analysis.code_offset;
        let block = &mut buf[offset..offset + *length];

        let key = match key_or_signature {
            KeyOrSignature::Key(key) => *key,
            KeyOrSignature::Signature(sig) => brute_force(block, sig)?,
        };

        decrypt(block, key);
    }

    fs::write(&config.out_path, &buf)?;
    Ok(())
}
