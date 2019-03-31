use basic_block::Builder;

fn main() {
    exercise_8_5_1();
}

fn exercise_8_5_1() {
    println!("Exercise 8.5.1:");

    let stmts = "d = b * c
e = a + b
b = b * c
a = e - d";

    let dag = Builder::parse(stmts);
    println!("{:#?}", dag);
}
