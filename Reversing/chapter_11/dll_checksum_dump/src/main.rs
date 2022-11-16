use std::{env, process};

use dll_checksum_dump::dump_sys_dll;

fn main() {
    let dll_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("{HELP}");
        process::exit(1);
    });

    dump_sys_dll(&dll_name).unwrap_or_else(|e| {
        eprintln!("{e}");
        eprintln!("{HELP}");
        process::exit(1);
    })
}

const HELP: &str = "USAGE: EXEC DLL_NAME
Dump export entries of 32-bit Windows system dlls";
