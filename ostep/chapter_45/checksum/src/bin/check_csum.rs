use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    process,
};

use anyhow::Context;
use checksum::{read_as_much, xor::Xor, Checksum};

fn main() {
    let args = argh::from_env();
    if let Err(e) = exec(args) {
        eprintln!("{:#}", e);
        process::exit(1);
    }
}

fn exec(args: CheckSumArgs) -> anyhow::Result<()> {
    const BLOCK_SIZE: usize = 4 * 1024;

    let mut input_fd =
        BufReader::new(File::open(&args.input).context("Failed to open input file")?);
    let csum_fd = BufReader::new(File::open(&args.csum).context("Failed to open checksum file")?);
    let mut csum_bytes = csum_fd.bytes();
    let mut checksum = Xor::new();
    let mut buf = [0u8; BLOCK_SIZE];

    loop {
        let amt = read_as_much(&mut input_fd, &mut buf)?;
        if amt == 0 {
            break;
        }

        let check_byte = if let Some(byte) = csum_bytes.next() {
            byte?
        } else {
            println!("Checksum exhausted before the input file");
            return Ok(());
        };

        checksum.clear();
        if checksum.digest(&buf[..amt])[0] != check_byte {
            let offset = input_fd.by_ref().stream_position()? - amt as u64;
            println!(
                "The block starting at offset {} doesn't match the pre-computed checksum",
                offset,
            );
            return Ok(());
        }
    }

    println!("No corruption detected");
    Ok(())
}

#[derive(argh::FromArgs)]
/// Check an input file against a pre-computed checksum.
struct CheckSumArgs {
    #[argh(option, short = 'i')]
    /// path to the input file to be checked
    input: String,
    #[argh(option, short = 'c')]
    /// path to the checksum file
    csum: String,
}
