use std::{
    fmt::{self, Display},
    str,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local};
use nix::{
    libc::{c_long, mode_t, time_t},
    sys::stat::{Mode, SFlag},
};
pub struct PermissionBits {
    flag: SFlag,
    mode: Mode,
}

impl PermissionBits {
    pub fn from_bits(bits: mode_t) -> Option<Self> {
        Some(Self {
            flag: SFlag::from_bits(bits & SFlag::S_IFMT.bits())?,
            mode: Mode::from_bits(bits & 0o777)?,
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
