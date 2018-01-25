extern crate des;
extern crate generic_array;
extern crate rand;

use des::{BlockCipher, Des};
use rand::Rng;
use generic_array::GenericArray;

fn main() {
    let mut rng = rand::thread_rng();
    let mut key1 = [0; 8];
    let mut key2 = [0; 8];
    let mut m1 = [0; 8];
    let mut m2 = [0; 8];

    for i in 0..8 {
        // key1 is random
        key1[i] = rng.gen::<u8>();
        // key2 is the complement of key1
        key2[i] = !key1[i];
        // message1 is random
        m1[i] = rng.gen::<u8>();
        // message2 is the complement of message1
        m2[i] = !m1[i];
    }

    let des1 = Des::new(GenericArray::from_slice(&key1));
    let des2 = Des::new(GenericArray::from_slice(&key2));
    des1.encrypt_block(GenericArray::from_mut_slice(&mut m1));
    des2.encrypt_block(GenericArray::from_mut_slice(&mut m2));

    let mut c = [0; 8];
    for i in 0..8 {
        c[i] = m1[i] ^ m2[i];
    }

    println!(
        "if complementation property holds, every byte in the xor of two ciphertexts will be 255"
    );
    println!("{:?}", c);
}
