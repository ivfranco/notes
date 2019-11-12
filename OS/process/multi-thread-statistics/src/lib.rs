use crossbeam_utils::thread;

pub fn multi_thread_statistics(input: &[u32]) -> (u32, u32, u32) {
    assert!(!input.is_empty());

    thread::scope(|s| {
        let avg = s.spawn(|_| input.iter().sum::<u32>() / input.len() as u32);
        let min = s.spawn(|_| input.iter().copied().min().unwrap());
        let max = s.spawn(|_| input.iter().copied().max().unwrap());

        (
            avg.join().unwrap(),
            min.join().unwrap(),
            max.join().unwrap(),
        )
    })
    .unwrap()
}

#[test]
fn stat_test() {
    assert_eq!(
        multi_thread_statistics(&[90, 81, 78, 95, 79, 72, 85]),
        (82, 72, 95)
    );
}
