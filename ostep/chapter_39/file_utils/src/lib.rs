use std::{
    ffi::CString,
    fmt::{self, Display},
    str,
    time::{Duration, SystemTime},
};

use anyhow::Context;
use chrono::{DateTime, Local};
use nix::{
    dir::Type,
    errno::{self, Errno},
    libc::{__error, c_long, dirent, mode_t, opendir, readdir, time_t, DIR},
    sys::stat::{Mode, SFlag},
};
pub struct PermissionBits {
    flag: SFlag,
    mode: Mode,
}

impl PermissionBits {
    pub fn from_bits(bits: mode_t) -> Option<Self> {
        Some(Self {
            flag: SFlag::from_bits_truncate(bits),
            mode: Mode::from_bits_truncate(bits),
        })
    }
}

impl Display for PermissionBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn rwx(mode: Mode, r: Mode, w: Mode, x: Mode) -> [u8; 3] {
            let mut buf = [b'-'; 3];
            if mode.contains(r) {
                buf[0] = b'r';
            }
            if mode.contains(w) {
                buf[1] = b'w';
            }
            if mode.contains(x) {
                buf[2] = b'x';
            }

            buf
        }

        let mut buf = [0u8; 1 + 3 * 3];

        let ty = if self.flag.contains(SFlag::S_IFDIR) {
            b'd'
        } else if self.flag.contains(SFlag::S_IFLNK) {
            b'l'
        } else {
            b'-'
        };
        buf[0] = ty;

        buf[1..1 + 3].copy_from_slice(&rwx(self.mode, Mode::S_IRUSR, Mode::S_IWUSR, Mode::S_IXUSR));

        buf[1 + 3..1 + 3 * 2].copy_from_slice(&rwx(
            self.mode,
            Mode::S_IRGRP,
            Mode::S_IWGRP,
            Mode::S_IXGRP,
        ));

        buf[1 + 3 * 2..1 + 3 * 3].copy_from_slice(&rwx(
            self.mode,
            Mode::S_IROTH,
            Mode::S_IWOTH,
            Mode::S_IXOTH,
        ));

        f.write_str(str::from_utf8(&buf).expect("valid ascii"))
    }
}

pub fn from_sec_nsec(sec: time_t, nsec: c_long) -> DateTime<Local> {
    let mut since_epoch = Duration::from_nanos(nsec as u64);
    since_epoch += Duration::from_secs(sec as u64);

    let epoch = SystemTime::UNIX_EPOCH;
    DateTime::from(epoch + since_epoch)
}

pub fn file_name(entry: &dirent) -> anyhow::Result<&str> {
    // entry.d_name is _NOT_ null terminated
    let name = &entry.d_name[..entry.d_namlen as usize];
    // # Safety
    // i8 and u8 must have the same memory layout. The pointer points to a valid buffer of size
    // entry.d_namelen.
    let name = unsafe { &*(name as *const [i8] as *const [u8]) };
    str::from_utf8(name).context("file name is not valid utf8")
}

pub fn file_type(mode: mode_t) -> Type {
    let sflag = SFlag::from_bits_truncate(mode);
    match sflag {
        _ if sflag.contains(SFlag::S_IFDIR) => Type::Directory,
        _ if sflag.contains(SFlag::S_IFLNK) => Type::Symlink,
        // ignore all other file type that's not regular file
        _ => Type::File,
    }
}

pub struct Dir {
    inner: *mut DIR,
}

impl Dir {
    pub fn open(path: &str) -> Result<Self, Errno> {
        // # Safety
        // The directory path is properly null-terminated.
        unsafe {
            // str in Rust is not null terminated
            let buf = path.bytes().collect::<Vec<_>>();
            let cname =
                CString::new(buf).expect("null terminator should be appended by the constructor");

            let dir = opendir(cname.as_ptr());
            if dir.is_null() {
                Err(errno::from_i32(*__error()))
            } else {
                Ok(Self { inner: dir })
            }
        }
    }
}

impl Iterator for Dir {
    type Item = Result<dirent, Errno>;

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
                    Some(Err(errno))
                }
            } else {
                Some(Ok(*entry))
            }
        }
    }
}
