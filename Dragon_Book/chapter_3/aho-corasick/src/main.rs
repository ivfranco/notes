use aho_corasick::{failure, fibonacci_string, kmp, Trie};

fn main() {
    exercise_3_4_3();
    exercise_3_4_6();
    exercise_3_4_9();
    exercise_3_4_11();
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

fn exercise_3_4_11() {
    println!("\nExercise 3.4.11:");

    let input_1: &[&[u8]] = &[b"aaa", b"abaaa", b"ababaaa"];
    let input_2: &[&[u8]] = &[b"all", b"fall", b"fatal", b"llama", b"lame"];
    let input_3: &[&[u8]] = &[b"pipe", b"pet", b"item", b"temper", b"perpetual"];

    println!("{:?}", Trie::new(input_1));
    println!("{:?}", Trie::new(input_2));
    println!("{:?}", Trie::new(input_3));
}
