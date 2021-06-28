use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
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

fn exec(args: CreateSumArgs) -> anyhow::Result<()> {
    const BLOCK_SIZE: usize = 4 * 1024;

    let mut input_fd =
        BufReader::new(File::open(&args.input).context("failed to open input file")?);
    let mut output_fd =
        BufWriter::new(File::create(&args.output).context("failed to create output file")?);
    let mut checksum = Xor::new();
    let mut buf = [0u8; BLOCK_SIZE];

    loop {
        let amt = read_as_much(&mut input_fd, &mut buf)?;
        if amt == 0 {
            break;
        }

        checksum.clear();
        let sum = checksum.digest(&buf[..amt]);
        output_fd.write_all(&sum)?;
    }

    output_fd.flush()?;
    Ok(())
}

#[derive(argh::FromArgs)]
/// Create checksum from a file.
struct CreateSumArgs {
    #[argh(positional)]
    /// path to a file
    input: String,

    #[argh(option, short = 'o')]
    /// which file the checksum is written to
    output: String,
}
