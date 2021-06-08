use std::{fmt::Display, path::Path, process, time::Duration};

use anyhow::Context;
use argh::FromArgs;
use chrono::{DateTime, Local};
use file_utils::PermissionBits;
use nix::{
    libc::{c_long, mode_t, time_t},
    sys::stat::stat,
};
use std::time::SystemTime;

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

fn mask(mode: mode_t) -> mode_t {
    const MASK: mode_t = 0o777;
    mode & MASK
}

fn from_sec_nsec(sec: time_t, nsec: c_long) -> DateTime<Local> {
    let mut since_epoch = Duration::from_nanos(nsec as u64);
    since_epoch += Duration::from_secs(sec as u64);

    let epoch = SystemTime::UNIX_EPOCH;
    DateTime::from(epoch + since_epoch)
}

#[derive(FromArgs)]
/// Display file status.
struct StatArgs {
    #[argh(positional)]
    file: String,
}
