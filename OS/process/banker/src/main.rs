use banker::Banker;

fn main() {
    problem_3();
}

fn problem_3() {
    println!("P3");

    let available = vec![1, 5, 2, 0];

    let allocation = vec![
        vec![0, 0, 1, 2],
        vec![1, 0, 0, 0],
        vec![1, 3, 5, 4],
        vec![0, 6, 3, 2],
        vec![0, 0, 1, 4],
    ];

    let max = vec![
        vec![0, 0, 1, 2],
        vec![1, 7, 5, 0],
        vec![2, 3, 5, 6],
        vec![0, 6, 5, 2],
        vec![0, 6, 5, 6],
    ];

    let mut banker = Banker::from_state(available, allocation, max);
    println!("{:?}", banker.need);

    if banker.safe() {
        println!("State is safe");
    } else {
        println!("State is unsafe");
    }

    println!("{:?}", banker.request(1, &[0, 4, 2, 0]));
}
