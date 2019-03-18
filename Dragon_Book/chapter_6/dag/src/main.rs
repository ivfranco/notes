use dag::env::Env;
use dag::symbolic;
use dag::symbolic::three_addr::ProcBuilder;
use dag::utils::Array;

fn main() {
    exercise_6_1_1();
    exercise_6_1_2();
    exercise_6_2_1();
    exercise_6_3_1();
    exercise_6_4_3();
    exercise_6_4_6();
    exercise_6_4_7();
    exercise_6_4_8();
    exercise_6_4_9();
}

fn exercise_6_1_1() {
    println!("Exercise 6.1.1:");

    println!(
        "{:?}",
        symbolic::DAG::parse("((x + y) - ((x + y) * (x - y))) + ((x + y) * (x - y))").unwrap()
    );
}

fn exercise_6_1_2() {
    println!("Exercise 6.1.2:");

    println!("{:?}", symbolic::DAG::parse("a + b + (a + b)").unwrap());
    println!("{:?}", symbolic::DAG::parse("a + b + a + b").unwrap());
    println!(
        "{:?}",
        symbolic::DAG::parse("a + a + (a + a + a + (a + a + a + a))").unwrap()
    );
}

fn exercise_6_2_1() {
    println!("Exercise 6.2.1:");

    println!("{:?}", symbolic::DAG::parse("a + -(b + c)").unwrap());
}

fn exercise_6_3_1() {
    println!("Exercise 6.3.1:");

    let decls = "float x;
record { float x; float y; } p;
record { int tag; float x; float y; } q;";

    println!("{:?}", Env::parse(decls).unwrap());
}

fn exercise_6_4_3() {
    println!("Exercise 6.4.3:");

    println!("{:#?}", ProcBuilder::parse("x = a[i] + b[j];").unwrap());
    println!(
        "{:#?}",
        ProcBuilder::parse("x = a[i][j] + b[i][j];").unwrap()
    );
    println!(
        "{:#?}",
        ProcBuilder::parse("x = a[b[i][j]][c[k]];").unwrap()
    );
}

fn exercise_6_4_6() {
    println!("Exercise 6.4.6:");

    let arr = Array::new(0, 4, &[(1, 10), (1, 20)]);
    println!("{}", arr.row_major(&[4, 5]));
    println!("{}", arr.row_major(&[10, 8]));
    println!("{}", arr.row_major(&[3, 17]));
}

fn exercise_6_4_7() {
    println!("Exercise 6.4.7:");

    let arr = Array::new(0, 4, &[(1, 10), (1, 20)]);
    println!("{}", arr.col_major(&[4, 5]));
    println!("{}", arr.col_major(&[10, 8]));
    println!("{}", arr.col_major(&[3, 17]));
}

fn exercise_6_4_8() {
    println!("Exercise 6.4.8:");

    let arr = Array::new(0, 8, &[(1, 4), (0, 4), (5, 10)]);
    println!("{}", arr.row_major(&[3, 4, 5]));
    println!("{}", arr.row_major(&[1, 2, 7]));
    println!("{}", arr.row_major(&[4, 3, 9]));
}

fn exercise_6_4_9() {
    println!("Exercise 6.4.9:");

    let arr = Array::new(0, 8, &[(1, 4), (0, 4), (5, 10)]);
    println!("{}", arr.col_major(&[3, 4, 5]));
    println!("{}", arr.col_major(&[1, 2, 7]));
    println!("{}", arr.col_major(&[4, 3, 9]));
}
