use aho_corasick::{failure, fibonacci_string, kmp};

fn main() {
    exercise_3_4_3();
    exercise_3_4_6();
    exercise_3_4_9();
}

fn exercise_3_4_3() {
    println!("\nExercise 3.4.3:");

    let patterns: &[&[u8]] = &[b"abababaab", b"aaaaaa", b"abbaabb"];

    for pattern in patterns {
        let s = std::str::from_utf8(pattern).unwrap();
        println!("failure function for {} is {:?}", s, failure(pattern));
    }
}

fn exercise_3_4_6() {
    println!("\nExercise 3.4.6:");

    let strings: &[&str] = &["abababaab", "abababbaa"];
    let pattern = "ababaa";

    for string in strings {
        match kmp(string.as_bytes(), pattern.as_bytes()) {
            Some(i) => println!(
                "{} contains an occurance of {} at index {}",
                string, pattern, i
            ),
            None => println!("{} contains no occurance of {}", string, pattern),
        }
    }
}

fn exercise_3_4_9() {
    println!("\nExercise 3.4.9:");

    println!("{:?}", failure(fibonacci_string(6).as_bytes()));
    println!("{:?}", failure(fibonacci_string(7).as_bytes()));
}
