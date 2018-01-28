extern crate crypto;

use crypto::aessafe::AesSafe256Encryptor;
use crypto::symmetriccipher::BlockEncryptor;

fn cbc_mac(key: &[u8], iv: &[u8], msg: &[u8], mac: &mut [u8]) {
    let aes = AesSafe256Encryptor::new(key);
    let mut plain = [0; 16];
    mac.copy_from_slice(iv);
    for b in msg.chunks(16) {
        plain.copy_from_slice(mac);
        for (i, byte) in plain.iter_mut().enumerate() {
            *byte ^= b[i];
        }
        aes.encrypt_block(&plain, mac);
    }
}

fn main() {
    let mut key = [0; 32];
    key[0] = 0x80;
    key[31] = 0x01;
    // the problem didn't specify an IV, using the first 16 bytes of the message as IV here
    let plain = [
        0x4d,
        0x41,
        0x43,
        0x73,
        0x20,
        0x61,
        0x72,
        0x65,
        0x20,
        0x76,
        0x65,
        0x72,
        0x79,
        0x20,
        0x75,
        0x73,
        0x65,
        0x66,
        0x75,
        0x6c,
        0x20,
        0x69,
        0x6e,
        0x20,
        0x63,
        0x72,
        0x79,
        0x70,
        0x74,
        0x6f,
        0x67,
        0x72,
        0x61,
        0x70,
        0x68,
        0x79,
        0x21,
        0x20,
        0x20,
        0x20,
        0x20,
        0x20,
        0x20,
        0x20,
        0x20,
        0x20,
        0x20,
        0x20,
    ];
    let iv = &plain[..16];
    let msg = &plain[16..];
    let mut mac = [0; 16];
    cbc_mac(&key, iv, msg, &mut mac);
    println!("{:?}", mac);
}
