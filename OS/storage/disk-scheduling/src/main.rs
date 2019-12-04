use disk_scheduling::{clook, cscan, fcfs, look, scan, seek_time, sstf, total_distance};
use rand::{thread_rng, Rng};

fn main() {
    problem_10_11();
    problem_10_12();
    problem_10_24();
}

fn problem_10_11() {
    println!("P10.11");

    let requests = &[2069, 1212, 2296, 2800, 544, 1618, 356, 1523, 4965, 3681];
    let init = 2150;
    let end = 4999;

    println!("FCFS: {}", total_distance(&fcfs(init, end, requests)));
    println!("SSTF: {}", total_distance(&sstf(init, end, requests)));
    println!("SCAN: {}", total_distance(&scan(init, end, requests)));
    println!("C-SCAN: {}", total_distance(&cscan(init, end, requests)));
    println!("LOOK: {}", total_distance(&look(init, end, requests)));
    println!("C-LOOK: {}", total_distance(&clook(init, end, requests)));
}

fn problem_10_12() {
    println!("P10.12");

    let requests = &[2069, 1212, 2296, 2800, 544, 1618, 356, 1523, 4965, 3681];
    let init = 2150;
    let end = 4999;

    println!("FCFS: {:.3}ms", seek_time(&fcfs(init, end, requests)));
    println!("SSTF: {:.3}ms", seek_time(&sstf(init, end, requests)));
    println!("SCAN: {:.3}ms", seek_time(&scan(init, end, requests)));
    println!("C-SCAN: {:.3}ms", seek_time(&cscan(init, end, requests)));
    println!("LOOK: {:.3}ms", seek_time(&look(init, end, requests)));
    println!("C-LOOK: {:.3}ms", seek_time(&clook(init, end, requests)));
}

fn problem_10_24() {
    println!("P10.24");
    let mut rng = thread_rng();
    let mut requests = vec![];
    let end = 4999;
    for _ in 0..=end {
        requests.push(rng.gen_range(0, end + 1));
    }
    let init = rng.gen_range(0, end + 1);

    println!("FCFS: {}", total_distance(&fcfs(init, end, &requests)));
    println!("SSTF: {}", total_distance(&sstf(init, end, &requests)));
    println!("SCAN: {}", total_distance(&scan(init, end, &requests)));
    println!("C-SCAN: {}", total_distance(&cscan(init, end, &requests)));
    println!("LOOK: {}", total_distance(&look(init, end, &requests)));
    println!("C-LOOK: {}", total_distance(&clook(init, end, &requests)));
}
