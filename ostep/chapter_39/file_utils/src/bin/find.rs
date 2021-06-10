use std::process;

fn main() {
    let args = argh::from_env();
    if let Err(e) = exec(args) {
        eprintln!("{:#}", e);
        process::exit(1);
    }
}

fn exec(args: FindArgs) -> anyhow::Result<()> {
    Ok(())
}

#[derive(argh::FromArgs)]
/// Walk a file directory.
struct FindArgs {
    #[argh(positional, default = "\".\".to_string()")]
    root: String,
}
