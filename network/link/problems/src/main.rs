use problems::{checksum, short_crc};
use textplots::{Chart, Plot, Shape};

fn main() {
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
    problem_12();
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

fn problem_12() {
    println!("\nP12");

    for &n in &[15, 25, 35] {
        println!("N = {}", n);

        let nf = n as f32;

        let slotted = (0 ..= 100)
            .map(|p| {
                let p = p as f32 / 100.0;
                let e = nf * p * (1.0 - p).powi(n - 1);
                (p, e)
            })
            .collect::<Vec<_>>();

        let pure = (0 ..= 100)
            .map(|p| {
                let p = p as f32 / 100.0;
                let e = nf * p * (1.0 - p).powi(2 * n - 2);
                (p, e)
            })
            .collect::<Vec<_>>();

        Chart::new(120, 60, 0.0, 1.0)
            .lineplot(Shape::Lines(&slotted))
            .lineplot(Shape::Lines(&pure))
            .display();
    }
}