extern crate crypto;

use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::digest::Digest;
use crypto::mac::Mac;
use crypto::aessafe::AesSafe256Encryptor;
use crypto::symmetriccipher::BlockEncryptor;
use std::mem::swap;

const KEY_SIZE: usize = 32;
const MAX_MSG_CNT: u32 = 0xffffffff;
const MAX_MSG_LEN: usize = 0xffffffff;
const MAX_EXT_LEN: usize = 0xffffffff;
const AES_BLOCK_SIZE: usize = 16;
const NONCE_LEN: usize = 4;
const MAC_LEN: usize = 32;

#[derive(PartialEq, Eq)]
enum Role {
    Send,
    Recv,
}

enum RecvError {
    AuthenticationFailure,
    MessageOrderError,
}

type Key = [u8; KEY_SIZE];

#[derive(Debug)]
struct Channel {
    key_send_enc: Key,
    key_rec_enc: Key,
    key_send_auth: Key,
    key_rec_auth: Key,
    msg_cnt_send: u32,
    msg_cnt_rec: u32,
}

fn derive_key(key: &[u8], phrase: &str, out: &mut [u8]) {
    assert!(key.len() == KEY_SIZE);
    assert!(out.len() == KEY_SIZE);

    let mut sha = Sha256::new();
    sha.input(&[0; 64]);
    sha.input(key);
    sha.input_str(phrase);
    sha.result(out);
    sha.reset();
    sha.input(out);
    sha.result(out);
}

fn hmac_sha_256(key: &[u8], msg: &[u8], out: &mut [u8]) {
    let mut hmac = Hmac::new(Sha256::new(), key);
    hmac.input(msg);
    hmac.raw_result(out);
}

fn authenticate(key: &[u8], nonce: &[u8], ext: &[u8], msg: &[u8], out: &mut [u8]) {
    assert!(out.len() == MAC_LEN);

    let mut auth: Vec<u8> = vec![];
    auth.extend_from_slice(&nonce);
    auth.extend_from_slice(&u32_to_byte_array(ext.len() as u32));
    auth.extend_from_slice(ext);
    auth.extend_from_slice(msg);
    hmac_sha_256(key, &auth, out);
    wipe_bytes(&mut auth);
}

fn u32_to_byte_array(n: u32) -> [u8; 4] {
    let mut bytes = [0; 4];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = (n >> (i * 8) & 0xff) as u8;
    }
    bytes
}

fn byte_array_to_u32(bytes: &[u8]) -> u32 {
    let mut n = bytes[3] as u32;
    for i in (0..3).rev() {
        n <<= 8;
        n += bytes[i] as u32;
    }
    n
}

fn generate_key_sequence(key: &[u8], nonce: &[u8], out: &mut [u8]) {
    assert!(out.len() % AES_BLOCK_SIZE == 0);

    let aes = AesSafe256Encryptor::new(key);
    let mut ctr = [0; AES_BLOCK_SIZE];
    (&mut ctr[NONCE_LEN..NONCE_LEN * 2]).copy_from_slice(&nonce);
    for (i, block) in out.chunks_mut(AES_BLOCK_SIZE).enumerate() {
        (&mut ctr[..NONCE_LEN]).copy_from_slice(&u32_to_byte_array(i as u32));
        aes.encrypt_block(&ctr, block);
    }
}

impl Channel {
    fn overhead() -> usize {
        NONCE_LEN + MAC_LEN
    }

    fn new(key: &[u8], role: Role) -> Self {
        assert!(key.len() == KEY_SIZE);

        let mut key_send_enc = [0; KEY_SIZE];
        let mut key_rec_enc = [0; KEY_SIZE];
        let mut key_send_auth = [0; KEY_SIZE];
        let mut key_rec_auth = [0; KEY_SIZE];

        derive_key(key, "Enc Alice to Bob", &mut key_send_enc);
        derive_key(key, "Enc Bob to Alice", &mut key_rec_enc);
        derive_key(key, "Auth Alice to Bob", &mut key_send_auth);
        derive_key(key, "Auth Bob to Alice", &mut key_rec_auth);

        if role == Role::Recv {
            swap(&mut key_send_enc, &mut key_rec_enc);
            swap(&mut key_send_auth, &mut key_rec_auth);
        }

        Channel {
            key_send_enc,
            key_rec_enc,
            key_send_auth,
            key_rec_auth,
            msg_cnt_send: 0,
            msg_cnt_rec: 0,
        }
    }

    fn send_message(&mut self, msg: &[u8], ext: &[u8], out: &mut [u8]) {
        assert!(self.msg_cnt_send < MAX_MSG_CNT);
        assert!(msg.len() <= MAX_MSG_LEN);
        assert!(ext.len() <= MAX_EXT_LEN);
        assert!(out.len() == msg.len() + NONCE_LEN + MAC_LEN);

        let (nonce_sector, tail) = out.split_at_mut(NONCE_LEN);
        let (mac_sector, cipher_sector) = tail.split_at_mut(MAC_LEN);

        self.msg_cnt_send += 1;
        nonce_sector.copy_from_slice(&u32_to_byte_array(self.msg_cnt_send));

        let key_sequence_length = (msg.len() / AES_BLOCK_SIZE + 1) * AES_BLOCK_SIZE;
        let mut key_sequence: Vec<u8> = vec![0; key_sequence_length];
        generate_key_sequence(&self.key_send_enc, nonce_sector, &mut key_sequence);

        for i in 0..msg.len() {
            cipher_sector[i] = msg[i] ^ key_sequence[i];
        }

        wipe_bytes(&mut key_sequence);

        authenticate(
            &self.key_send_auth,
            nonce_sector,
            ext,
            cipher_sector,
            mac_sector,
        );
    }

    fn receive_message(
        &mut self,
        cipher: &[u8],
        ext: &[u8],
        msg: &mut [u8],
    ) -> Result<(), RecvError> {
        assert!(cipher.len() > NONCE_LEN + MAC_LEN);
        assert!(ext.len() <= MAX_EXT_LEN);
        assert!(cipher.len() == msg.len() + NONCE_LEN + MAC_LEN);

        let (nonce_sector, tail) = cipher.split_at(NONCE_LEN);
        let (mac_sector, cipher_sector) = tail.split_at(MAC_LEN);

        let cnt = byte_array_to_u32(nonce_sector);
        if cnt <= self.msg_cnt_rec {
            return Err(RecvError::MessageOrderError);
        }

        let mut mac = [0; MAC_LEN];
        authenticate(
            &self.key_rec_auth,
            nonce_sector,
            ext,
            cipher_sector,
            &mut mac,
        );

        if &mac != mac_sector {
            return Err(RecvError::AuthenticationFailure);
        }

        let key_sequence_length = (msg.len() / AES_BLOCK_SIZE + 1) * AES_BLOCK_SIZE;
        let mut key_sequence = vec![0; key_sequence_length];
        generate_key_sequence(&self.key_rec_enc, nonce_sector, &mut key_sequence);

        for i in 0..msg.len() {
            msg[i] = cipher_sector[i] ^ key_sequence[i];
        }

        wipe_bytes(&mut key_sequence);

        self.msg_cnt_rec = cnt;
        Ok(())
    }
}

fn wipe_bytes(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        *byte = 0u8;
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        wipe_bytes(&mut self.key_send_enc);
        wipe_bytes(&mut self.key_rec_enc);
        wipe_bytes(&mut self.key_send_auth);
        wipe_bytes(&mut self.key_rec_auth);
    }
}

fn main() {
    let mut key = [0; KEY_SIZE];
    let mut alice = Channel::new(&key, Role::Send);
    let mut bob = Channel::new(&key, Role::Recv);
    wipe_bytes(&mut key);

    let plain = "cryptographicsystemsareextremelydifficulttobuildneverthelessf\
                 orsomereasonmanynonexpertsinsistondesigningnewencryptionschem\
                 esthatseemtothemtobemoresecurethananyotherschemeonearththeunf\
                 ortunatetruthhoweveristhatsuchschemesareusuallytrivialtobreak";
    let msg = plain.as_bytes();
    let cipher_length = msg.len() + Channel::overhead();
    let mut cipher = vec![0; cipher_length];
    alice.send_message(msg, &[], &mut cipher);
    let mut msg_recv = vec![0; msg.len()];
    bob.receive_message(&cipher, &[], &mut msg_recv);

    println!{"{}\n{:?}\n{:?}", plain, cipher, String::from_utf8(msg_recv)};
}
