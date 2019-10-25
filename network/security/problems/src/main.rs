use std::collections::HashMap;

fn main() {
    problem_1();
    problem_4();
}

const PLAIN_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const CIPHER_ALPHABET: &str = "mnbvcxzasdfghjklpoiuytrewq";

fn translate(cipher: &HashMap<char, char>, text: &str) -> String {
    text.chars()
        .map(|c| cipher.get(&c.to_ascii_lowercase()).cloned().unwrap_or(c))
        .collect::<String>()
}

fn problem_1() {
    println!("\nP1");

    println!(
        "{}",
        translate(
            &PLAIN_ALPHABET
                .chars()
                .zip(CIPHER_ALPHABET.chars())
                .collect(),
            "This is an easy problem"
        )
    );
    println!(
        "{}",
        translate(
            &CIPHER_ALPHABET
                .chars()
                .zip(PLAIN_ALPHABET.chars())
                .collect(),
            "rmij'u uamu xyj"
        )
    );
}

fn block_cipher(plain: &[u8], n: usize, scrambler: bool) -> Vec<u8> {
    let mut ciphertext = plain.to_vec();
    for _ in 0 .. n {
        for byte in ciphertext.iter_mut() {
            *byte = byte.reverse_bits();
        }

        if scrambler {
            ciphertext.reverse();
        }
    }

    ciphertext
}

fn problem_4() {
    println!("\nP4");

    let mut plain = vec![0b1010_0000; 8];

    println!("{:02X?}", block_cipher(&plain, 3, false));
    println!("{:02X?}", block_cipher(&plain, 3, true));

    plain[7] = 0b1010_0001;

    println!("{:02X?}", block_cipher(&plain, 3, false));
    println!("{:02X?}", block_cipher(&plain, 3, true));
}