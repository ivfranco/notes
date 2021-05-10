use std::io::Write;
use std::io::{stdout, BufRead, BufReader};
use std::{env, error::Error, fmt::Display};
use std::{fs::File, process};

fn main() {
    let mut args = env::args();
    let path = args
        .nth(1)
        .unwrap_or_else(|| error_exit("MISSING FILE PATH"));

    if let Err(e) = parse(&path) {
        error_exit(e);
    }
}

fn parse(path: &str) -> Result<(), Box<dyn Error>> {
    // each page is 4KB
    const OFFSET_BITS: u32 = 12;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let stdout = stdout();
    let mut lock = stdout.lock();
    let mut first = true;

    for line in reader.lines().take(10_000) {
        let line = line?;

        // anything not a memory access starts with two equal signs
        if line.starts_with("==") {
            continue;
        }

        let suffix = line
            .trim_start_matches(|c: char| c.is_whitespace() || matches!(c, 'I' | 'S' | 'L' | 'M'));

        let (address, _) = suffix.split_at(suffix.find(',').ok_or("No comma")?);
        let page = u64::from_str_radix(address, 16)? >> OFFSET_BITS;

        if first {
            first = false;
            write!(lock, "{}", page)?;
        } else {
            write!(lock, ",{}", page)?;
        }
    }

    Ok(())
}

fn error_exit(err: impl Display) -> ! {
    eprintln!("USAGE: EXEC LACKEY_TRACE_PATH");
    eprintln!("{}", err);
    process::exit(1);
}
