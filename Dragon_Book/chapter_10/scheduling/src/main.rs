use scheduling::Binary;

fn main() {
    exercise_10_3_1();
}

fn binary_a() -> Binary {
    Binary::parse(
        "
LD R1, a
LD R2, b
SUB R3, R1, R2
ADD R2, R1, R2
ST a, R3
ST b, R2
    ",
    )
    .unwrap()
}

fn binary_b() -> Binary {
    Binary::parse(
        "
LD R1, a
LD R2, b
SUB R1, R1, R2
ADD R2, R1, R2
ST a, R1
ST b, R2
    ",
    )
    .unwrap()
}

fn binary_c() -> Binary {
    Binary::parse(
        "
LD R1, a
LD R2, b
SUB R3, R1, R2
ADD R4, R1, R2
ST a, R3
ST b, R4
    ",
    )
    .unwrap()
}

fn exercise_10_3_1() {
    println!("Exercise 10.3.1:");
    for binary in &[binary_a(), binary_b(), binary_c()] {
        println!("{:#?}", binary.dependency_graph(|_, _| ()));
    }
}
