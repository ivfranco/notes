use multi_thread_statistics::multi_thread_statistics;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let input = args
        .map(|arg| arg.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let (avg, min, max) = multi_thread_statistics(&input);

    println!("The average value is {}", avg);
    println!("The minimum value is {}", min);
    println!("The maximum value is {}", max);
}
