#![deny(missing_docs)]
//! Decrypt encrypted code section in Defender.exe, the resulting executable is no longer a valid
//! crackme but still executable and useful for reversing purposes.

use std::{error::Error, fs};

use argh::FromArgs;
use sha1_smol::Sha1;

use crate::decrypt::{brute_force, decrypt};

mod decrypt;

fn as_byte_array(v: &toml::Value) -> Option<Vec<u8>> {
    v.as_array()?
        .iter()
        .map(|v| -> Option<u8> {
            let b = v.as_integer()?;
            Some(b as u8)
        })
        .collect()
}

#[derive(Debug)]
enum KeyOrSignature {
    Key(u32),
    Signature(Vec<u8>),
}

#[derive(Debug)]
struct Block {
    base: usize,
    length: usize,
    key_or_signature: KeyOrSignature,
    enc_dec_jump: usize,
    jump_rewrite: Vec<u8>,
    critical: bool,
}

#[derive(Debug)]
struct Watchdog {
    base: usize,
    rewrite: Vec<u8>,
}

impl Watchdog {
    fn interpret(v: &toml::Value) -> Option<Self> {
        let base = v.get("base")?.as_integer()? as usize;
        let rewrite = as_byte_array(v.get("rewrite")?)?;

        Some(Self { base, rewrite })
    }
}

impl Block {
    fn interpret(v: &toml::Value) -> Option<Self> {
        let base = v.get("base")?.as_integer()? as usize;
        let length = v.get("length")?.as_integer()? as usize;
        let key = v.get("key").and_then(|v| {
            let i = v.as_integer()?;
            Some(i as u32)
        });
        let signature = v.get("signature").and_then(as_byte_array);

        let key_or_signature = match (key, signature) {
            (Some(key), _) => KeyOrSignature::Key(key),
            (_, Some(sig)) => KeyOrSignature::Signature(sig),
            _ => return None,
        };

        let enc_dec_jump = v.get("enc_dec_jump")?.as_integer()? as usize;
        let jump_rewrite = as_byte_array(v.get("jump_rewrite")?)?;
        let critical = v.get("critical").and_then(|v| v.as_bool()).unwrap_or(false);

        Some(Block {
            base,
            length,
            key_or_signature,
            enc_dec_jump,
            jump_rewrite,
            critical,
        })
    }
}

struct Analysis {
    exec_hash: String,
    code_offset: usize,
    blocks: Vec<Block>,
    watchdog: Watchdog,
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
            .ok_or("key 'code_offset' not found or malformed, expected u32")?;

        let blocks: Vec<Block> = analysis
            .get("blocks")
            .and_then(|v| {
                v.as_array()?
                    .iter()
                    .map(Block::interpret)
                    .collect::<Option<_>>()
            })
            .ok_or("key 'blocks' not found or malformed, expected array")?;

        let watchdog = analysis.get("watchdog")
            .and_then(Watchdog::interpret)
            .ok_or("key 'watchdog' not found or malformed, expected integer 'watchdog.base' and array 'watchdog.rewrite'")?;

        Ok(Self {
            exec_hash,
            code_offset,
            blocks,
            watchdog,
        })
    }
}

#[derive(FromArgs)]
/// Decrypt encrypted code section in Defender.exe, the resulting executable is no longer a valid
/// crackme but still executable and useful for reversing purposes.
pub struct Config {
    /// path to Defender.exe
    #[argh(positional)]
    in_path: String,
    /// path to output decrypted executable
    #[argh(positional)]
    out_path: String,
    /// whether to preserve critical sections, which if decrypted the program will no longer be
    /// functionally equivalent to Defender.exe, false by default
    #[argh(switch, short = 'p')]
    preserve_critical: bool,
}

/// Decrypt Defender.exe code section according to analysis file. Would attempt a brute force attack
/// when key is not given for an encrypted block.
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
        enc_dec_jump,
        jump_rewrite,
        critical,
    } in &analysis.blocks
    {
        // decrypt code section
        if !critical || !config.preserve_critical {
            let offset = base - analysis.code_offset;
            let block = &mut buf[offset..offset + *length];

            let key = match key_or_signature {
                KeyOrSignature::Key(key) => *key,
                KeyOrSignature::Signature(sig) => brute_force(block, sig)?,
            };

            decrypt(block, key);

            // skip decryption / encryption code
            let offset = enc_dec_jump - analysis.code_offset;
            let instr_buf = &mut buf[offset..offset + jump_rewrite.len()];
            instr_buf.copy_from_slice(jump_rewrite);
        }
    }

    // neutralize watchdog thread
    let watchdog = &analysis.watchdog;
    let offset = watchdog.base - analysis.code_offset;
    let instr_buf = &mut buf[offset..offset + watchdog.rewrite.len()];
    instr_buf.copy_from_slice(&watchdog.rewrite);

    fs::write(&config.out_path, &buf)?;
    Ok(())
}
