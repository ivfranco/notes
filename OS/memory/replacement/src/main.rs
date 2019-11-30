use rand::{thread_rng, Rng};
use replacement::{Replacement, CNT, FIFO, LRU, OPT};

fn main() {
    problem_9_8();
    problem_9_21();
    problem_9_30();
    problem_9_39();
}

fn problem_9_8() {
    println!("P9.8");

    let references = &[1, 2, 3, 4, 2, 1, 5, 6, 2, 1, 2, 3, 7, 6, 3, 2, 1, 2, 3, 6];
    for frames in 1..=7 {
        println!("{} physical frames", frames);
        let mut fifo = FIFO::new(frames);
        println!(
            "    FIFO: {} page faults",
            count_page_faults(references, &mut fifo)
        );
        let mut lru = LRU::new(frames);
        println!(
            "    LRU: {} page faults",
            count_page_faults(references, &mut lru)
        );
        let mut opt = OPT::new(frames);
        println!(
            "    OPT: {} page faults",
            count_page_faults(references, &mut opt)
        );
    }
}

fn count_page_faults<R>(references: &[u32], replacement: &mut R) -> usize
where
    R: Replacement,
{
    let mut count = 0;
    for idx in 0..references.len() {
        if replacement.allocate(references, idx) {
            count += 1;
        }
    }
    count
}

fn problem_9_21() {
    println!("P9.21");

    let references = &[7, 2, 3, 1, 2, 5, 3, 4, 6, 7, 7, 1, 0, 5, 4, 6, 2, 3, 0, 1];
    let frames = 3;
    println!("{} physical frames", frames);
    let mut fifo = FIFO::new(frames);
    println!(
        "    FIFO: {} page faults",
        count_page_faults(references, &mut fifo)
    );
    let mut lru = LRU::new(frames);
    println!(
        "    LRU: {} page faults",
        count_page_faults(references, &mut lru)
    );
    let mut opt = OPT::new(frames);
    println!(
        "    OPT: {} page faults",
        count_page_faults(references, &mut opt)
    );
}

fn problem_9_30() {
    println!("P9.30");

    let references = &[
        1, 2, 3, 4, 5, 3, 4, 1, 6, 7, 8, 7, 8, 9, 7, 8, 9, 5, 4, 5, 4, 2,
    ];
    let frames = 4;
    println!("{} physical frames", frames);
    let mut cnt = CNT::new(frames);
    println!(
        "CNT: {} page faults",
        count_page_faults(references, &mut cnt)
    );
    let mut opt = OPT::new(frames);
    println!(
        "OPT: {} page faults",
        count_page_faults(references, &mut opt)
    );
}

fn problem_9_39() {
    println!("P9.39");

    let mut rng = thread_rng();
    let mut references = [0; 100];
    for page in references.iter_mut() {
        *page = rng.gen_range(0, 10);
    }
    for frames in 1..=7 {
        println!("{} physical frames", frames);
        let mut fifo = FIFO::new(frames);
        println!(
            "    FIFO: {} page faults",
            count_page_faults(&references, &mut fifo)
        );
        let mut lru = LRU::new(frames);
        println!(
            "    LRU: {} page faults",
            count_page_faults(&references, &mut lru)
        );
        let mut opt = OPT::new(frames);
        println!(
            "    OPT: {} page faults",
            count_page_faults(&references, &mut opt)
        );
    }
}
