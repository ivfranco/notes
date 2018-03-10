use nix;
use std::os::unix::io::RawFd;
use nix::errno::Errno;
use nix::unistd::*;
use nix::Error::Sys;
use std::slice;
use std::cmp::min;

const RIO_BUFSIZE: usize = 8192;
pub struct RioFd {
    rio_fd: RawFd,
    rio_cnt: usize,
    rio_off: usize,
    rio_buf: [u8; RIO_BUFSIZE],
}

impl RioFd {
    pub fn new(fd: RawFd) -> Self {
        let buf = [0; RIO_BUFSIZE];
        RioFd {
            rio_fd: fd,
            rio_cnt: 0,
            rio_off: 0,
            rio_buf: buf,
        }
    }
}

pub fn rio_readn(fd: RawFd, usrbuf: &mut [u8], n: usize) -> nix::Result<usize> {
    let mut nleft = n;
    let mut bufp = usrbuf.as_mut_ptr();
    unsafe {
        while nleft > 0 {
            let nread = read(fd, slice::from_raw_parts_mut(bufp, nleft)).or_else(|e| match e {
                nix::Error::Sys(Errno::EINTR) => Ok(0),
                _ => Err(e),
            })?;
            if nread == 0 {
                break;
            }
            nleft -= nread;
            bufp = bufp.offset(nread as isize);
        }

        Ok(n - nleft)
    }
}

pub fn rio_writen(fd: RawFd, usrbuf: &[u8], n: usize) -> nix::Result<usize> {
    let mut nleft = n;
    let mut bufp = &usrbuf[..nleft];

    while nleft > 0 {
        let nwritten = write(fd, bufp).or_else(|e| match e {
            Sys(Errno::EINTR) => Ok(0),
            _ => Err(e),
        })?;
        nleft -= nwritten;
        bufp = &bufp[nwritten..];
    }

    Ok(n)
}

pub fn rio_read(rp: &mut RioFd, usrbuf: &mut [u8], n: usize) -> nix::Result<usize> {
    // will only execute once unless interrupted by EINTR
    while rp.rio_cnt == 0 {
        match read(rp.rio_fd, &mut rp.rio_buf) {
            // redo read if interrupted by signal handlers
            Err(Sys(Errno::EINTR)) => continue,
            // throw other exceptions
            e @ Err(_) => return e,
            Ok(n) => rp.rio_cnt = n,
        }
        if rp.rio_cnt == 0 {
            return Ok(0);
        } else {
            rp.rio_off = 0;
        }
    }

    let cnt = min(n, rp.rio_cnt);
    usrbuf.copy_from_slice(&rp.rio_buf[rp.rio_off..rp.rio_off + cnt]);
    rp.rio_off += cnt;
    rp.rio_cnt -= cnt;

    Ok(cnt)
}

// nightly api
fn from_ref_mut<T>(s: &mut T) -> &mut [T] {
    unsafe { slice::from_raw_parts_mut(s, 1) }
}

pub fn rio_readlineb(rp: &mut RioFd, usrbuf: &mut [u8], maxlen: usize) -> nix::Result<usize> {
    let mut c: u8 = 0;
    let mut n = 1;
    while n < maxlen {
        match rio_read(rp, from_ref_mut(&mut c), 1)? {
            1 => {
                usrbuf[n - 1] = c;
                if c == '\n' as u8 {
                    break;
                }
            }
            0 => {
                if n == 1 {
                    return Ok(0);
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
        n += 1;
    }
    usrbuf[n] = 0;

    Ok(n)
}

pub fn rio_readnb(rp: &mut RioFd, usrbuf: &mut [u8], n: usize) -> nix::Result<usize> {
    let mut nleft = n;
    let mut bufp = usrbuf.as_mut_ptr();
    unsafe {
        while nleft > 0 {
            let nread =
                rio_read(rp, slice::from_raw_parts_mut(bufp, nleft), nleft).or_else(|e| match e {
                    nix::Error::Sys(Errno::EINTR) => Ok(0),
                    _ => Err(e),
                })?;
            if nread == 0 {
                break;
            }
            nleft -= nread;
            bufp = bufp.offset(nread as isize);
        }

        Ok(n - nleft)
    }
}
