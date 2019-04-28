use critical_path::critical_path_len;

fn main() {
    exercise_10_2_4();
    exercise_10_2_5();
}

fn exercise_10_2_4() {
    println!("Exercise 10.2.4:");

    let edges = &[
        (1, 3),
        (1, 7),
        (1, 11),
        (2, 3),
        (2, 4),
        (2, 6),
        (2, 7),
        (2, 8),
        (2, 10),
        (2, 11),
        (3, 4),
        (3, 6),
        (3, 7),
        (3, 8),
        (3, 10),
        (3, 11),
        (4, 6),
        (4, 7),
        (4, 8),
        (4, 10),
        (4, 11),
        (5, 6),
        (5, 9),
        (5, 10),
        (6, 7),
        (6, 8),
        (6, 9),
        (6, 10),
        (6, 11),
        (7, 8),
        (7, 10),
        (7, 11),
        (8, 10),
        (8, 11),
        (9, 10),
        (10, 11),
    ];

    println!("{} steps", critical_path_len(edges));
}

fn exercise_10_2_5() {
    println!("Exercise 10.2.4:");

    let edges = &[(0, 2), (0, 3), (0, 6), (1, 2), (2, 4), (3, 4), (5, 6)];
    println!("{} steps", critical_path_len(edges));
}
