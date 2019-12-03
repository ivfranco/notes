pub fn fcfs(init: i32, _end: i32, requests: &[i32]) -> Vec<i32> {
    let mut schedule = vec![init];
    schedule.extend(requests);
    schedule
}

pub fn sstf(mut init: i32, _end: i32, requests: &[i32]) -> Vec<i32> {
    let mut requests = requests.to_vec();
    let mut schedule = vec![init];
    while let Some(i) = closest(init, &requests) {
        let request = requests.remove(i);
        schedule.push(request);
        init = request;
    }
    schedule
}

fn closest(target: i32, requests: &[i32]) -> Option<usize> {
    requests
        .iter()
        .enumerate()
        .min_by_key(|(_, &r)| (r - target).abs())
        .map(|(i, _)| i)
}

pub fn scan(init: i32, end: i32, requests: &[i32]) -> Vec<i32> {
    let mut schedule = vec![init];
    let mut inner = requests.iter().filter(|&&r| r >= init).collect::<Vec<_>>();
    if !inner.is_empty() {
        // there's requests inner to the initial position
        // scan to the innermost of them
        inner.sort();
        schedule.extend(inner);
    }
    let mut outer = requests.iter().filter(|&&r| r < init).collect::<Vec<_>>();
    if !outer.is_empty() {
        // there's requests outer to the initial position
        // scan to the end then back to the outermost of them
        schedule.push(end);
        outer.sort_by(|a, b| a.cmp(b).reverse());
        schedule.extend(outer);
    }

    schedule
}

pub fn cscan(init: i32, end: i32, requests: &[i32]) -> Vec<i32> {
    let mut schedule = vec![init];
    let mut inner = requests.iter().filter(|&&r| r >= init).collect::<Vec<_>>();
    if !inner.is_empty() {
        // there's requests inner to the initial position
        // scan to the innermost of them
        inner.sort();
        schedule.extend(requests.iter().filter(|&&r| r >= init));
    }
    let mut outer = requests.iter().filter(|&&r| r < init).collect::<Vec<_>>();
    if !outer.is_empty() {
        // there's requests outer to the initial position
        // scan to the end, then start, then back to the outermost of them
        schedule.push(end);
        schedule.push(0);
        outer.sort();
        schedule.extend(outer);
    }

    schedule
}

pub fn look(init: i32, _end: i32, requests: &[i32]) -> Vec<i32> {
    let mut schedule = vec![init];
    let mut inner = requests.iter().filter(|&&r| r >= init).collect::<Vec<_>>();
    if !inner.is_empty() {
        // there's requests inner to the initial position
        // scan to the innermost of them
        inner.sort();
        schedule.extend(inner);
    }
    let mut outer = requests.iter().filter(|&&r| r < init).collect::<Vec<_>>();
    if !outer.is_empty() {
        // there's requests outer to the initial position
        // scan back to the outermost of them
        outer.sort_by(|a, b| a.cmp(b).reverse());
        schedule.extend(outer);
    }

    schedule
}

pub fn clook(init: i32, _end: i32, requests: &[i32]) -> Vec<i32> {
    let mut schedule = vec![init];
    let mut inner = requests.iter().filter(|&&r| r >= init).collect::<Vec<_>>();
    if !inner.is_empty() {
        // there's requests inner to the initial position
        // scan to the innermost of them
        inner.sort();
        schedule.extend(inner);
    }
    let mut outer = requests.iter().filter(|&&r| r < init).collect::<Vec<_>>();
    if !outer.is_empty() {
        // there's requests outer to the initial position
        // scan back to the outermost, then the innermost of them
        outer.sort();
        schedule.extend(outer);
    }

    schedule
}

pub fn total_distance(schedule: &[i32]) -> i32 {
    schedule
        .iter()
        .zip(schedule.iter().skip(1))
        .map(|(curr, next)| (next - curr).abs())
        .sum()
}

pub fn seek_time(schedule: &[i32]) -> f64 {
    const X: f64 = 0.756;
    const Y: f64 = 0.244;

    schedule
        .iter()
        .zip(schedule.iter().skip(1))
        .map(|(curr, next)| {
            let d = f64::from((next - curr).abs());
            X + Y * d.sqrt()
        })
        .sum()
}
