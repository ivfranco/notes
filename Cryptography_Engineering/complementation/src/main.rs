extern crate des;
extern crate generic_array;

use des::{BlockCipher, Des};
use generic_array::GenericArray;

fn main() {
    let key1 = GenericArray::from_slice(&[0; 8]);
    let des1 = Des::new(key1);
    let key2 = GenericArray::from_slice(&[255; 8]);
    let des2 = Des::new(key2);
    let mut m1 = [0; 8];
    let mut m2 = [255; 8];
    des1.encrypt_block(GenericArray::from_mut_slice(&mut m1));
    des2.encrypt_block(GenericArray::from_mut_slice(&mut m2));
    println!("{:?}", m1);
    println!("{:?}", m2);
    let mut c = [0; 8];
    for i in 0..8 {
        c[i] = m1[i] ^ m2[i];
    }
    println!("{:?}", c);
}
