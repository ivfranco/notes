fn main() {
    problem_3_3();
    problem_3_4();
}

// 1-byte internet checksum
fn u8_checksum(bytes: &[u8]) -> u8 {
    let mut acc = 0u16;
    let high_mask = 0xff00u16;
    for &byte in bytes {
        print!("{:0>8b} + {:0>8b}", acc as u8, byte);
        acc += u16::from(byte);
        if acc & high_mask != 0 {
            // overflow, wrap to least significant bit
            acc += 1;
            acc &= !high_mask;
        }
        println!(" = {:0>8b}", acc as u8);
    }

    let complement = !(acc as u8);
    println!("complement of the sum is {:0>8b}", complement);
    complement
}

fn problem_3_3() {
    println!("\nP3.3");
    println!(
        "{:0>8b}",
        u8_checksum(&[0b0101_0011, 0b0110_0110, 0b0111_0100])
    );
}

fn problem_3_4() {
    println!("\nP3.4");
    println!(
        "{:0>8b}",
        u8_checksum(&[0b0101_1100, 0b0110_0101]),
    );
    println!(
        "{:0>8b}",
        u8_checksum(&[0b1101_1010, 0b0110_0101]),
    );
}