use replacement::{Replacement, FIFO, LRU, OPT};

fn main() {
    problem_9_8();
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
