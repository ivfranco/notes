use std::error::Error;

use argh::from_env;
use defender_decrypt::{unpack, Config};

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = from_env();
    unpack(&config)
}
