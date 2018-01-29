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

type Key = [u8; 32];

#[derive(Debug)]
struct Channel {
    key_send_enc: Key,
    key_rec_enc: Key,
    key_send_auth: Key,
    key_rec_auth: Key,
    msg_cnt_send: u32,
    msg_cnt_rec: u32,
}

fn shad_256(input: &[u8], output: &mut [u8]) {
    let mut sha = Sha256::new();
    sha.input(&[0; 64]);
    sha.input(&input);
    sha.result(output);
    sha.reset();
    sha.input(&output[..32]);
    sha.result(output);
}


fn derive_key(key: &[u8], phrase: &str, output: &mut [u8]) {
    let mut key_phrase: Vec<u8> = Vec::from(key);
    key_phrase.extend_from_slice(phrase.as_bytes());
    shad_256(&key_phrase, output);
}

fn hmac_sha_256(key: &[u8], msg: &[u8], out: &mut [u8]) {
    let mut hmac = Hmac::new(Sha256::new(), key);
    hmac.input(msg);
    hmac.raw_result(out);
}

fn authenticate(key: &[u8], nonce: &[u8], ext: &[u8], msg: &[u8], out: &mut [u8]) {
    let mut auth: Vec<u8> = vec![];
    auth.extend_from_slice(&nonce);
    auth.extend_from_slice(&u32_to_byte_array(ext.len() as u32));
    auth.extend_from_slice(ext);
    auth.extend_from_slice(msg);
    hmac_sha_256(key, &auth, out);
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
    let aes = AesSafe256Encryptor::new(key);
    let mut ctr = [0; AES_BLOCK_SIZE];
    for (i, block) in out.chunks_mut(AES_BLOCK_SIZE).enumerate() {
        (&mut ctr[..NONCE_LEN]).copy_from_slice(&u32_to_byte_array(i as u32));
        (&mut ctr[NONCE_LEN..NONCE_LEN * 2]).copy_from_slice(&nonce);
        aes.encrypt_block(&ctr, block);
    }
}

impl Channel {
    fn new(key: &[u8], role: Role) -> Self {
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

        self.msg_cnt_send += 1;
        let nonce = u32_to_byte_array(self.msg_cnt_send);

        let key_sequence_length = (msg.len() / AES_BLOCK_SIZE + 1) * AES_BLOCK_SIZE;
        let mut key_sequence: Vec<u8> = vec![0; key_sequence_length];
        generate_key_sequence(&self.key_send_enc, &nonce, &mut key_sequence);

        for i in 0..msg.len() {
            out[(NONCE_LEN + MAC_LEN) + i] = msg[i] ^ key_sequence[i];
        }

        let mut mac = [0; MAC_LEN];
        authenticate(
            &self.key_send_auth,
            &nonce,
            ext,
            &out[NONCE_LEN + MAC_LEN..],
            &mut mac,
        );

        (&mut out[NONCE_LEN..(NONCE_LEN + MAC_LEN)]).copy_from_slice(&mac);
        (&mut out[..NONCE_LEN]).copy_from_slice(&nonce);
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

        let cnt = byte_array_to_u32(&cipher[..NONCE_LEN]);
        if cnt <= self.msg_cnt_rec {
            return Err(RecvError::MessageOrderError);
        }

        let mut mac = [0; MAC_LEN];
        authenticate(
            &self.key_rec_auth,
            &cipher[..NONCE_LEN],
            ext,
            &cipher[(NONCE_LEN + MAC_LEN)..],
            &mut mac,
        );

        if &mac != &cipher[NONCE_LEN..(NONCE_LEN + MAC_LEN)] {
            return Err(RecvError::AuthenticationFailure);
        }

        let key_sequence_length =
            ((cipher.len() - NONCE_LEN - MAC_LEN) / AES_BLOCK_SIZE + 1) * AES_BLOCK_SIZE;
        let mut key_sequence = vec![0; key_sequence_length];
        generate_key_sequence(&self.key_rec_enc, &cipher[..NONCE_LEN], &mut key_sequence);

        for i in 0..(cipher.len() - NONCE_LEN - MAC_LEN) {
            msg[i] = cipher[i + NONCE_LEN + MAC_LEN] ^ key_sequence[i];
        }

        Ok(())
    }
}

fn main() {
    let key = [0; KEY_SIZE];
    let mut alice = Channel::new(&key, Role::Send);
    let mut bob = Channel::new(&key, Role::Recv);

    let plain = "Ahora";
    let msg = plain.as_bytes();
    let mut msg_recv = vec![0; msg.len()];
    let cipher_length = msg.len() + NONCE_LEN + MAC_LEN;
    let mut cipher = vec![0; cipher_length];
    alice.send_message(msg, &[], &mut cipher);
    bob.receive_message(&cipher, &[], &mut msg_recv);

    println!{"{}\n{:?}\n{:?}", plain, cipher, String::from_utf8(msg_recv)};
}
