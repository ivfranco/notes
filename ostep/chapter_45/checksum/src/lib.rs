pub mod additive;
pub mod crc;
pub mod fletcher;
pub mod xor;

use std::{
    fs::File,
    io::{self, Read},
};

/// A trait abstracting the common behaviors of checksum algorithms. The output must be an array of
/// bytes of length N.
pub trait Checksum<const N: usize> {
    fn write(&mut self, bytes: &[u8]);
    fn finish(&self) -> [u8; N];
    fn clear(&mut self);

    fn digest(&mut self, bytes: &[u8]) -> [u8; N] {
        self.write(bytes);
        self.finish()
    }

    fn write_from<R: Read>(&mut self, mut reader: R) -> io::Result<()> {
        let mut buf = [0u8; 4 * 1024];

        loop {
            let amt = reader.read(&mut buf)?;
            if amt == 0 {
                return Ok(());
            }

            self.write(&buf[..amt]);
        }
    }
}

/// Feed an input file to the checksum algorithm. If the argument is "-", the input file will be
/// read from the standard input, otherwise `arg` will be interpreted as the path to the input file.
pub fn checksum_of<C, const N: usize>(arg: &str, mut checksum: C) -> io::Result<[u8; N]>
where
    C: Checksum<N>,
{
    if arg == "-" {
        let stdin = io::stdin();
        checksum.write_from(stdin.lock())?;
    } else {
        let file = File::open(arg)?;
        checksum.write_from(file)?;
    }

    Ok(checksum.finish())
}

/// Compute the checksum of an input file, from standard input or a path, and output the checksum as
/// binary numbers separated by a space.
pub fn main_common<C, const N: usize>(checksum: C)
where
    C: Checksum<N>,
{
    use std::env;
    use std::process;

    let mut args = env::args();
    let path = args.nth(1).unwrap_or_else(|| {
        eprintln!("Missing argument");
        eprintln!("{}", HELP);
        process::exit(1);
    });

    for byte in &checksum_of(&path, checksum).unwrap() {
        print!("{:0>8b} ", byte);
    }
    println!();
}

const HELP: &str = "
Usage: cmdname <file>

Arguments:
    <file>      either path to a file, or '-' and the program will read input from stdin
";

/// Read `buf.len()` bytes to the buffer unless the reader is exhausted.
pub fn read_as_much<R>(mut reader: R, buf: &mut [u8]) -> std::io::Result<usize>
where
    R: Read,
{
    let mut filled = 0;
    loop {
        let amt = reader.read(&mut buf[filled..])?;
        if amt == 0 {
            break;
        }
        filled += amt;
        if filled == buf.len() {
            break;
        }
    }

    Ok(filled)
}
