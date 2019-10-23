use problems::{delay_and_deviation, queue_simulate, RoundRobin, WFQ, LeakyBucket};
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    problem_12();
    problem_18();
    problem_19();
    problem_20();
    problem_21();
}

fn problem_12() {
    println!("\nP12");

    let (delays, deviations) = delay_and_deviation(
        &[1, 2, 3, 4, 5, 6, 7, 8],
        &[8, 9, 12, 12, 12, 15, 15, 16],
        0.1,
    );

    println!("{:.3?}", delays);
    println!("{:.3?}", deviations);
}

fn report_delay(arrivals: &[u32], departures: &[u32]) {
    println!("Departure times: {:?}", departures);
    let delays = arrivals
        .iter()
        .zip(departures)
        .map(|(a, d)| d - a)
        .collect::<Vec<_>>();
    println!("Delays: {:?}", delays);
    println!(
        "Avg delay: {:.3}",
        f64::from(delays.iter().sum::<u32>()) / delays.len() as f64
    );
}

fn problem_18() {
    println!("\nP18");

    let arrivals = &[0, 0, 1, 1, 2, 3, 3, 5, 5, 7, 8, 8];

    println!("FIFO");
    let fifo_classes = vec![0; arrivals.len()];
    report_delay(
        arrivals,
        &queue_simulate(VecDeque::new(), arrivals, &fifo_classes),
    );

    println!("Priority Queue");
    let pq_classes = &[0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    report_delay(
        arrivals,
        &queue_simulate(BinaryHeap::new(), arrivals, pq_classes),
    );

    println!("Round Robin");
    let rr_classes = &[0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1];
    report_delay(
        arrivals,
        &queue_simulate(RoundRobin::new(2), arrivals, rr_classes),
    );

    println!("WFQ");
    report_delay(
        arrivals,
        &queue_simulate(WFQ::new(2.0), arrivals, pq_classes),
    )
}

fn problem_19() {
    println!("\nP19");

    let arrivals = &[0, 0, 1, 1, 2, 3, 3, 5, 5, 7, 8, 8];
    let classes = &[0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1];

    println!("Priority Queue");
    report_delay(
        arrivals,
        &queue_simulate(BinaryHeap::new(), arrivals, classes),
    );

    println!("Round Robin");
    report_delay(
        arrivals,
        &queue_simulate(RoundRobin::new(2), arrivals, classes),
    );

    println!("WFQ");
    report_delay(
        arrivals,
        &queue_simulate(WFQ::new(0.5), arrivals, classes),
    );
}

fn report_bucket_simulation(bucket: &mut LeakyBucket, arrivals: &[u32]) {
    let mut t = 0;
    let mut i = 0;

    while !bucket.is_empty() || i < arrivals.len() {
        while i < arrivals.len() {
            if arrivals[i] <= t {
                bucket.push(i);
                i += 1;
            } else {
                break;
            }
        }

        print!("t = {}, {:?}", t, bucket);
        println!(", output = {:?}", bucket.advance());

        t += 1;
    }
}

const BUCKET_ARRIVALS: &[u32] = &[0,0,0,1,2,3,6,6,7,7];

fn problem_20() {
    println!("\nP20");
    report_bucket_simulation(&mut LeakyBucket::new(1, 2), BUCKET_ARRIVALS);
}

fn problem_21() {
    println!("\nP21");
    report_bucket_simulation(&mut LeakyBucket::new(2, 2), BUCKET_ARRIVALS);
}