use problems::{checksum, short_crc};

fn main() {
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

fn bin(checksum: [u8; 2]) -> String {
    format!("{:08b} {:08b}", checksum[0], checksum[1])
}

fn problem_3() {
    println!("\nP3");

    println!("{}", bin(checksum(b"Networking")));
}

fn problem_4() {
    println!("\nP4");

    println!("{}", bin(checksum(&(1 ..= 10).collect::<Vec<_>>())));
    println!("{}", bin(checksum(b"BCDEFGHIJK")));
    println!("{}", bin(checksum(b"bcdefghijk")));
}

fn problem_5() {
    println!("\nP5");

    println!("{:04b}", short_crc(0b10_1010_1010, 0b10011, 4));
}

fn problem_6() {
    println!("\nP6");

    println!("{:04b}", short_crc(0b10_0101_0101, 0b10011, 4));
    println!("{:04b}", short_crc(0b01_0110_1010, 0b10011, 4));
    println!("{:04b}", short_crc(0b10_1010_0000, 0b10011, 4));
}

fn parity(n: u64) -> bool {
    n.count_ones() % 2 == 0
}

fn problem_7() {
    println!("\nP7");

    for data in 0b1000 ..= 0b1111 {
        println!("{:04b} XOR 1001 = {:04b}", data, data ^ 0b1001);
        assert_eq!(parity(data), parity(data ^ 0b1001));
    }
}