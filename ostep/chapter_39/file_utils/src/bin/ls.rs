use std::{ffi::CString, fs::Permissions, process, str};

use anyhow::{anyhow, bail, Context};
use nix::{
    errno,
    libc::{__error, dirent, opendir, readdir, DIR},
};

use file_utils::PermissionBits;

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
            long_list(entry)?;
        } else {
            short_list(entry)?;
        }
    }
    println!();

    Ok(())
}

fn long_list(entry: dirent) -> anyhow::Result<()> {
    Ok(())
}

fn short_list(entry: dirent) -> anyhow::Result<()> {
    // entry.d_name is _NOT_ null terminated
    let name = &entry.d_name[..entry.d_namlen as usize];
    let name = unsafe { &*(name as *const [i8] as *const [u8]) };
    print!(
        "{}\t",
        str::from_utf8(name).context("file name not valid utf8")?
    );
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

struct Dir {
    inner: *mut DIR,
}

impl Dir {
    fn open(path: &str) -> anyhow::Result<Self> {
        // # Safety
        // The directory path is properly null-terminated.
        unsafe {
            // str in Rust is not null terminated
            let buf = path.bytes().collect::<Vec<_>>();
            let cname =
                CString::new(buf).expect("null terminator should be appended by the constructor");

            let dir = opendir(cname.as_ptr());
            if dir.is_null() {
                bail!("opendir() failed")
            } else {
                Ok(Self { inner: dir })
            }
        }
    }
}

impl Iterator for Dir {
    type Item = anyhow::Result<dirent>;

    fn next(&mut self) -> Option<Self::Item> {
        // # Safety
        // The global __error is thread-specific.
        // The returned reference to `dirent` is valid until the next call to readdir(), the struct
        // is copied to a stack variable before that.
        unsafe {
            // reset global error number
            *__error() = 0;

            let entry = readdir(self.inner);
            if entry.is_null() {
                if *__error() == 0 {
                    None
                } else {
                    // an error triggered by readdir, as the error number is reset at the beginning
                    let errno = errno::from_i32(*__error());
                    Some(Err(anyhow!("readdir() failed {}", errno)))
                }
            } else {
                Some(Ok(*entry))
            }
        }
    }
}
