use std::{fmt::Display, path::Path, process};

use anyhow::Context;
use argh::FromArgs;
use file_utils::{from_sec_nsec, PermissionBits};
use nix::sys::stat::stat;

fn main() {
    let args = argh::from_env();
    if let Err(e) = exec(args) {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn exec(args: StatArgs) -> anyhow::Result<()> {
    let path = Path::new(&args.file);
    let stat = stat(path).context("syscall stat() failed")?;

    list(
        "File",
        path.file_name()
            .context("invalid path")?
            .to_str()
            .context("not valid utf8")?,
    );
    list("Size", stat.st_size);
    list("Blocks", stat.st_blocks);
    list("Links", stat.st_nlink);
    list("Device", stat.st_dev);

    list("Uid", stat.st_uid);
    list("Gid", stat.st_gid);

    list(
        "Access",
        PermissionBits::from_bits(stat.st_mode).expect("valid mode_t returned by syscall"),
    );

    let atime = from_sec_nsec(stat.st_atime, stat.st_atime_nsec);
    list("Access", atime);
    let mtime = from_sec_nsec(stat.st_mtime, stat.st_mtime_nsec);
    list("Modify", mtime);
    let ctime = from_sec_nsec(stat.st_ctime, stat.st_ctime_nsec);
    list("Change", ctime);

    Ok(())
}

fn list(key: impl Display, value: impl Display) {
    println!("{:>10}: {}", key, value)
}

#[derive(FromArgs)]
/// Display file status.
struct StatArgs {
    #[argh(positional)]
    file: String,
}
