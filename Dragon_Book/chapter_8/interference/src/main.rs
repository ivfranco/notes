use interference::{Block, Interference};

fn main() {
    exercise_8_8_1();
}

fn exercise_8_8_1() {
    println!("Exercise 8.8.1:");

    let b1 = Block::parse(
        "a = b + c
d = d - b
e = a + f",
        "acdef",
    );
    let b2 = Block::parse("f = a - d", "cdef");
    let b3 = Block::parse(
        "b = d + f
e = a - c",
        "bcdef",
    );
    let b4 = Block::parse("b = d + c", "bcdef");

    let mut graph = Interference::new();

    for block in [b1, b2, b3, b4].iter() {
        graph.update(block);
    }

    println!("{:#?}", graph);
}
