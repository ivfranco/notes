use std::{path::Path, process, str};

use anyhow::Context;
use nix::{
    fcntl::readlink,
    libc::{dirent, DT_LNK},
    sys::stat::lstat,
    unistd::{Gid, Uid},
};

use file_utils::{file_name, from_sec_nsec, Dir, PermissionBits};

fn main() {
    let args = argh::from_env();
    if let Err(e) = exec(args) {
        eprintln!("{:#}", e);
        process::exit(1);
    }
}

fn exec(args: LsArgs) -> anyhow::Result<()> {
    let dir = Dir::open(&args.directory)?;

    for entry in dir {
        let entry = entry?;
        if args.long {
            long_list(&args, &entry)?;
        } else {
            short_list(&entry)?;
        }
    }
    println!();

    Ok(())
}

fn long_list(args: &LsArgs, entry: &dirent) -> anyhow::Result<()> {
    let dir_path = Path::new(&args.directory);
    let file_name = file_name(entry)?;
    let file_path = dir_path.join(file_name);

    let stat = lstat(&file_path).context("stat() failed")?;

    let permission_bits =
        PermissionBits::from_bits(stat.st_mode).expect("valid mode_t returned by syscall");

    let uid = Uid::from_raw(stat.st_uid);
    let user = nix::unistd::User::from_uid(uid).context("getpwid_r() failed")?;
    let gid = Gid::from_raw(stat.st_gid);
    let group = nix::unistd::Group::from_gid(gid).context("getgrgid_r() failed")?;

    let mtime = from_sec_nsec(stat.st_mtime, stat.st_mtime_nsec);

    print!(
        "{perm} {link}\t{user}\t{group}\t{size}\t{date}\t{name}",
        perm = permission_bits,
        link = stat.st_nlink,
        user = user.map_or_else(|| uid.to_string(), move |user| user.name),
        group = group.map_or_else(|| gid.to_string(), move |group| group.name),
        size = stat.st_size,
        date = mtime.format("%h %e %R"),
        name = file_name,
    );

    if entry.d_type == DT_LNK {
        let target = readlink(&file_path).context("readlink() failed")?;
        print!(
            " -> {}",
            target.to_str().context("link target is not valid utf8")?,
        );
    }

    println!();

    Ok(())
}

fn short_list(entry: &dirent) -> anyhow::Result<()> {
    print!("{}\t", file_name(entry)?);
    Ok(())
}

#[derive(argh::FromArgs)]
/// List directory contents.
struct LsArgs {
    #[argh(switch, short = 'l')]
    /// list in long format
    long: bool,
    #[argh(positional, default = "default_dir()")]
    directory: String,
}

fn default_dir() -> String {
    ".".to_string()
}
