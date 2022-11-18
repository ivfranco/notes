use std::{
    error::Error,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use memchr::memmem;

pub(crate) fn decrypt(block: &mut [u8], key: u32) {
    // # Safety
    //
    // Any 4-byte pattern is a valid u32 value.
    let (prefix, pbs, _) = unsafe { block.align_to_mut::<u32>() };
    if prefix.is_empty() {
        decrypt_aligned(pbs, key);
    } else {
        decrypt_unaligned(block, key);
    }
}

fn decrypt_aligned(pbs: &mut [u32], key: u32) {
    let mut pb_last = 0;
    for pb in pbs.iter_mut() {
        let pb_current = *pb;
        *pb ^= pb_last ^ key;
        pb_last = pb_current;
    }
}

fn decrypt_unaligned(block: &mut [u8], key: u32) {
    let mut pb_last = 0u32;

    for chunk in block.chunks_exact_mut(4) {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(chunk);
        let pb = u32::from_ne_bytes(buf);
        chunk.copy_from_slice(&(pb ^ pb_last ^ key).to_ne_bytes());
        pb_last = pb;
    }
}

#[allow(dead_code)]
fn encrypt(block: &mut [u8], key: u32) {
    let mut cb_last = 0;
    for chunk in block.chunks_exact_mut(4) {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(chunk);
        let cb = u32::from_ne_bytes(buf) ^ key ^ cb_last;
        chunk.copy_from_slice(&cb.to_ne_bytes());
        cb_last = cb;
    }
}

pub(crate) fn brute_force(block: &[u8], signature: &[u8]) -> Result<u32, Box<dyn Error>> {
    let search = |tx: Sender<u32>, kill_switch: Arc<Mutex<bool>>, low: u32, high: u32| {
        const RESOLUTION: u32 = 2u32.pow(20);

        let mut buf = block.to_vec();

        for key in low..=high {
            if key % RESOLUTION == 0 && *kill_switch.lock().unwrap() {
                return;
            }

            decrypt(&mut buf, key);
            if memmem::find(&buf, signature).is_some() {
                tx.send(key).expect("main thread shouldn't panic");
                return;
            }

            buf.copy_from_slice(block);
        }
    };

    let p = thread::available_parallelism()?.get() as u32 * 2;

    println!("brute forcing with {p} threads");

    let key = thread::scope(|s| {
        let kill_switch = Arc::new(Mutex::new(false));
        let (tx, rx) = mpsc::channel();

        for i in 0..p {
            let thread_tx = tx.clone();
            let thread_kill_switch = Arc::clone(&kill_switch);
            let low = i * (u32::MAX / p);
            let high = if i == p - 1 {
                u32::MAX
            } else {
                low + (u32::MAX / p) - 1
            };

            s.spawn(move || {
                search(thread_tx, thread_kill_switch, low, high);
            });
        }

        let key = rx.recv().expect("search threads shouldn't panic");
        *kill_switch.lock().unwrap() = true;
        key
    });

    println!("key = {:x}", key);

    Ok(key)
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use super::*;

    #[test]
    fn consistency() {
        let mut rng = StdRng::from_entropy();

        for _ in 0..32 {
            let mut buf = [0u8; 0x1000];
            rng.fill(&mut buf);
            let key = rng.gen::<u32>();

            let mut enc_dec = buf;
            encrypt(&mut enc_dec, key);
            decrypt(&mut enc_dec, key);

            assert_eq!(buf, enc_dec);
        }
    }
}
