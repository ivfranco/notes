use nix::{
    fcntl::OFlag,
    sys::{
        mman::{mmap, shm_open, shm_unlink, MapFlags, ProtFlags},
        stat::Mode,
        wait::wait,
    },
    unistd::{fork, ftruncate},
};
use std::{env, process, ptr::null_mut};

fn main() {
    let mut args = env::args();
    args.next();

    let collatz = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("Error: input is not a non-negative integer");
            process::exit(1);
        });

    if let Err(err) = collatz_child_fork(collatz) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

const SHM_LEN: usize = 0x1000;
const SHM_NAME: &str = "collatz";

fn collatz_child_fork(mut collatz: u32) -> nix::Result<()> {
    let shm_fd = shm_open(SHM_NAME, OFlag::O_CREAT | OFlag::O_RDWR, Mode::all())?;
    ftruncate(shm_fd, SHM_LEN as i64)?;
    unsafe {
        // assume the pointer returned is aligned
        let mut ptr = mmap(
            null_mut(),
            SHM_LEN,
            ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            shm_fd,
            0,
        )?
        .cast::<u32>();

        if fork()?.is_parent() {
            wait()?;
            loop {
                let collatz = ptr.read();
                println!("{}", collatz);
                if collatz == 1 {
                    break;
                } else {
                    ptr = ptr.add(1);
                }
            }

            shm_unlink(SHM_NAME)?;
        } else {
            loop {
                ptr.write(collatz);

                if collatz == 1 {
                    break;
                } else if collatz % 2 == 0 {
                    collatz /= 2;
                } else {
                    collatz = collatz * 3 + 1;
                }

                ptr = ptr.add(1);
            }
        }
    }

    Ok(())
}
