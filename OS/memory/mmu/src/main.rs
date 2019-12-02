use mmu::{Addr, MMUResponse, MMU};
use std::{
    env,
    fs::{self, File},
    io,
};
use MMUResponse::*;

const BACKING_STORE_PATH: &str = "BACKING_STORE.bin";

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Usage: EXEC INPUT_FILE"))?;

    let input = fs::read_to_string(input_file_path)?;
    let references = input
        .lines()
        .map(|word| word.parse::<Addr>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Error: illegal address in data")
        })?;

    translate(&references)
}

fn translate(references: &[Addr]) -> io::Result<()> {
    let backing_store = File::open(BACKING_STORE_PATH)?;
    let mut mmu = MMU::new(backing_store);

    let mut tlb_hit = 0;
    let mut page_fault = 0;

    for (i, &addr) in references.iter().enumerate() {
        let byte = match mmu.access(addr, i)? {
            TLBHit(byte) => {
                tlb_hit += 1;
                byte
            }
            PageTableHit(byte) => byte,
            PageFault(byte) => {
                page_fault += 1;
                byte
            }
        };

        println!("0x{:04X}: {}", addr, byte as i8);
    }

    println!(
        "TLB hit ratio: {}",
        tlb_hit as f64 / references.len() as f64
    );
    println!(
        "page fault rate: {}",
        page_fault as f64 / references.len() as f64
    );

    Ok(())
}
