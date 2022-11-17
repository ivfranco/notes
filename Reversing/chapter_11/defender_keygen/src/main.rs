use std::{env, process};

use defender_keygen::{get_volume_serial, keygen};

fn main() {
    let user_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("USAGE: EXEC USER_NAME");
        process::exit(1);
    });

    println!(
        "{:<24} {:0>8X}",
        "Volume serial number:",
        get_volume_serial()
    );
    println!("{:<24} {}", "User name:", user_name);
    println!(
        "{:<24} {:0>16X}",
        "Defender serial number:",
        keygen(&user_name)
    );
}
