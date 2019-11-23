use banker::Banker;

fn main() {
    problem_3();
    problem_22();
    problem_23();
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

fn problem_22() {
    println!("P22");

    let allocation = vec![
        vec![3, 0, 1, 4],
        vec![2, 2, 1, 0],
        vec![3, 1, 2, 1],
        vec![0, 5, 1, 0],
        vec![4, 2, 1, 2],
    ];

    let max = vec![
        vec![5, 1, 1, 7],
        vec![3, 2, 1, 1],
        vec![3, 3, 2, 1],
        vec![4, 6, 1, 2],
        vec![6, 3, 2, 5],
    ];

    for available in vec![vec![0, 3, 0, 1], vec![1, 0, 0, 2]] {
        print!("{:?}, ", available);
        let banker = Banker::from_state(available, allocation.clone(), max.clone());
        println!("{}", banker.safe());
    }
}

fn problem_23() {
    println!("P23");

    let allocation = vec![
        vec![2, 0, 0, 1],
        vec![3, 1, 2, 1],
        vec![2, 1, 0, 3],
        vec![1, 3, 1, 2],
        vec![1, 4, 3, 2],
    ];

    let max = vec![
        vec![4, 2, 1, 2],
        vec![5, 2, 5, 2],
        vec![2, 3, 1, 6],
        vec![1, 4, 2, 4],
        vec![3, 6, 6, 5],
    ];

    let available = vec![3, 3, 2, 1];

    let banker = Banker::from_state(available, allocation, max);
    banker.safe();
    println!("{:?}", banker.clone().request(1, &[1, 1, 0, 0]));
    println!("{:?}", banker.clone().request(4, &[0, 0, 2, 0]));
}
