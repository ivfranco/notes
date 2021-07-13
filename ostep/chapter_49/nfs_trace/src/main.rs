use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

use nfs_trace::{Trace, Ty};

fn main() {
    let mut args = env::args();
    let trace_file = args.nth(1).unwrap_or_else(|| {
        eprintln!("USAGE: EXEC TRACE_FILE_PATH");
        process::exit(1);
    });
    let mut analyzers: Vec<Box<dyn Analyzer>> = vec![
        Box::new(Question3::new()),
        Box::new(Question4::new()),
        Box::new(Question5::new()),
        Box::new(Question6::new()),
        Box::new(Question7::new()),
    ];
    let reader = BufReader::new(File::open(trace_file).unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        let trace = Trace::parse(&line).unwrap();
        for analyzer in analyzers.iter_mut() {
            analyzer.feed(&trace);
        }
    }

    for analyzer in analyzers {
        analyzer.report();
    }
}

trait Analyzer {
    fn feed(&mut self, trace: &Trace);
    fn report(&self);
}

#[derive(Default)]
struct Question3 {
    size_sum: u64,
    access: u64,
    access_per_user: HashMap<String, u64>,
}

impl Question3 {
    fn new() -> Self {
        Self::default()
    }
}

impl Analyzer for Question3 {
    fn feed(&mut self, trace: &Trace) {
        if !matches!(trace.ty, Ty::Reply(..)) || trace.operation != "getattr" {
            return;
        }

        let user = trace.to.clone();
        let size = u64::from_str_radix(&trace.params["size"], 16).unwrap();

        self.access += 1;
        self.size_sum += size;
        *self.access_per_user.entry(user).or_default() += 1;
    }

    fn report(&self) {
        println!("\nQuestion 3");
        println!(
            "Average file size by access: {}",
            self.size_sum / self.access,
        );
        let mut vec = self.access_per_user.iter().collect::<Vec<_>>();
        vec.sort_by_key(|(_, access)| *access);
        vec.reverse();

        let total_users = vec.len();
        vec.retain(|(_, access)| **access >= self.access / 1000);

        println!(
            "Among all {} users, only {} users made more than 0.1% getattr requests",
            total_users,
            vec.len()
        );

        println!("List of requests made by top 10 users:");

        for (user, access) in vec.iter().take(10) {
            println!("    User {} send {} requests", user, access);
        }
    }
}

#[derive(Default)]
struct Question4 {
    access_per_file: HashMap<String, u64>,
    sequential: u64,
    non_sequential: u64,
}

impl Question4 {
    fn new() -> Self {
        Self::default()
    }
}

impl Analyzer for Question4 {
    fn feed(&mut self, trace: &Trace) {
        if !matches!(trace.ty, Ty::Request)
            || !(trace.operation == "read" || trace.operation == "write")
        {
            return;
        }

        let fh = trace.params["fh"].to_string();
        let off = u64::from_str_radix(&trace.params["off"], 16).unwrap();
        let count = u64::from_str_radix(&trace.params["count"], 16).unwrap();

        let end = self.access_per_file.entry(fh).or_default();
        if *end == off {
            self.sequential += 1;
        } else {
            self.non_sequential += 1;
        }

        *end = off + count;
    }

    fn report(&self) {
        println!("\nQuestion 4");
        println!("{} read / write access are sequential", self.sequential);
        println!(
            "{} read / write access are non-sequential",
            self.non_sequential
        );
        println!(
            "{:.2}% access are sequential",
            (self.sequential as f64 / (self.sequential + self.non_sequential) as f64) * 100.0
        );
    }
}

#[derive(Default)]
struct Question5 {
    request_per_client: HashMap<String, u64>,
    request: u64,
}

impl Question5 {
    fn new() -> Self {
        Self::default()
    }
}

impl Analyzer for Question5 {
    fn feed(&mut self, trace: &Trace) {
        if !matches!(trace.ty, Ty::Request) {
            return;
        }

        self.request += 1;

        if let Some(count) = self.request_per_client.get_mut(&trace.from) {
            *count += 1;
        } else {
            self.request_per_client.insert(trace.from.to_string(), 1);
        }
    }

    fn report(&self) {
        println!("\nQuestion 5");

        let mut vec = self.request_per_client.iter().collect::<Vec<_>>();
        vec.sort_by_key(|(_, count)| *count);
        vec.reverse();

        let total_clients = vec.len();
        vec.retain(|(_, count)| **count > self.request / 1000);
        println!(
            "Among all {} clients, only {} of them made more than 0.1% of all the requests",
            total_clients,
            vec.len()
        );

        println!("List top 10 clients");

        for (client, count) in vec.iter().take(10) {
            println!("    Client {} made {} requests", client, count);
        }
    }
}

#[derive(Default)]
struct Question6 {
    request_time: HashMap<u32, f64>,
    latencies: Vec<f64>,
}

impl Question6 {
    fn new() -> Self {
        Self::default()
    }
}

impl Analyzer for Question6 {
    fn feed(&mut self, trace: &Trace) {
        match trace.ty {
            Ty::Request => {
                self.request_time.insert(trace.session_id, trace.epoch);
            }
            Ty::Reply(_) => {
                if let Some(start) = self.request_time.remove(&trace.session_id) {
                    self.latencies.push(trace.epoch - start);
                }
                // ignore replies without the corresponding request
            }
        }
    }

    fn report(&self) {
        println!("\nQuestion 6");

        let min = self
            .latencies
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        println!("Min latency: {:.4}", min);
        let max = self
            .latencies
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        println!("Max latency: {:.4}", max);
        let avg = self.latencies.iter().sum::<f64>() / self.latencies.len() as f64;
        println!("Avg latency: {:.4}", avg);
    }
}

#[derive(Default)]
struct Question7 {
    lost_request: HashSet<u32>,
}

impl Question7 {
    fn new() -> Self {
        Self::default()
    }
}

impl Analyzer for Question7 {
    fn feed(&mut self, trace: &Trace) {
        match trace.ty {
            Ty::Request => {
                self.lost_request.insert(trace.session_id);
            }
            Ty::Reply(_) => {
                self.lost_request.remove(&trace.session_id);
            }
        }
    }

    fn report(&self) {
        println!("\nQuestion7");

        println!("{} requests are not replied to", self.lost_request.len());
        println!("List session id of 10 lost requests");

        for session_id in self.lost_request.iter().take(10) {
            println!("    {:x}", session_id);
        }
    }
}
