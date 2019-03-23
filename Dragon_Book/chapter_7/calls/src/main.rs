use calls::fibonacci::{fib0, fibonacci};
use calls::quicksort::quicksort;

fn main() {
    exercise_7_2_1();
    exercise_7_2_2();
    exercise_7_2_3();
    exercise_7_3_1();
}

fn exercise_7_2_1() {
    println!("Exercise 7.2.1:");

    let tree = quicksort(0, &mut [9, 8, 7, 6, 5, 4, 3, 2, 1]);
    println!("{:?}", tree);
    println!("depth: {:?}", tree.depth());
}

fn exercise_7_2_2() {
    println!("Exercise 7.2.2:");

    let tree = quicksort(0, &mut [1, 3, 5, 7, 9, 2, 4, 6, 8]);
    println!("{:?}", tree);
    println!("depth: {:?}", tree.depth());
}

fn exercise_7_2_3() {
    println!("Exercise 7.2.2:");

    println!("{:?}", fibonacci(5).1);
}

fn exercise_7_3_1() {
    println!("Exercise 7.3.1:");

    println!("{:?}", fib0(4).1);
}
