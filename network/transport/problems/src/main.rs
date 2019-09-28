fn main() {
    problem_3_3();
    problem_3_4();
    problem_3_31();
    problem_3_50();
    problem_3_51();
}

// 1-byte internet checksum
fn u8_checksum(bytes: &[u8]) -> u8 {
    let mut acc = 0u16;
    let high_mask = 0xff00u16;
    for &byte in bytes {
        print!("{:0>8b} + {:0>8b}", acc as u8, byte);
        acc += u16::from(byte);
        if acc & high_mask != 0 {
            // overflow, wrap to least significant bit
            acc += 1;
            acc &= !high_mask;
        }
        println!(" = {:0>8b}", acc as u8);
    }

    let complement = !(acc as u8);
    println!("complement of the sum is {:0>8b}", complement);
    complement
}

fn problem_3_3() {
    println!("\nP3.3");
    println!(
        "{:0>8b}",
        u8_checksum(&[0b0101_0011, 0b0110_0110, 0b0111_0100])
    );
}

fn problem_3_4() {
    println!("\nP3.4");
    println!("{:0>8b}", u8_checksum(&[0b0101_1100, 0b0110_0101]),);
    println!("{:0>8b}", u8_checksum(&[0b1101_1010, 0b0110_0101]),);
}

fn problem_3_31() {
    println!("\nP3.31");

    const ALPHA: f64 = 0.125;
    const BETA: f64 = 0.25;

    let mut estimated_rtt = 100;
    let mut dev_rtt = 5;

    for &rtt in &[106, 120, 140, 90, 115] {
        estimated_rtt = ((1.0 - ALPHA) * f64::from(estimated_rtt) + ALPHA * f64::from(rtt)) as i32;
        dev_rtt = ((1.0 - BETA) * f64::from(dev_rtt)
            + BETA * f64::from((rtt - estimated_rtt).abs())) as i32;
        println!("after observed sample rtt = {}ms, EstimatedRTT = {}ms, DevRTT = {}ms, TimeoutInterval = {}ms",
            rtt,
            estimated_rtt,
            dev_rtt,
            estimated_rtt + 4 * dev_rtt,
        )
    }
}

mod simulate {
    const SECOND: u32 = 1000;

    pub struct Host {
        cwnd: u32,
        rtt: u32,
        elapsed: u32,
    }

    impl Host {
        pub fn new(cwnd: u32, rtt: u32) -> Self {
            Host {
                cwnd,
                rtt,
                elapsed: 0,
            }
        }

        fn speed(&self) -> u32 {
            self.cwnd * SECOND / self.rtt
        }

        fn advance(&mut self, duration: u32) {
            self.elapsed += duration;
        }

        fn adjust(&mut self, lost: bool) {
            if self.elapsed >= self.rtt {
                self.elapsed -= self.rtt;
                if lost {
                    self.lost();
                } else {
                    self.ack();
                }
            }
        }

        fn lost(&mut self) {
            if self.cwnd > 1 {
                self.cwnd = (f64::from(self.cwnd) / 2.0).round() as u32;
            }
        }

        fn ack(&mut self) {
            self.cwnd += 1;
        }
    }

    pub struct Simulator {
        // per second
        link_capacity: u32,
        hosts: Vec<Host>,
    }

    impl Simulator {
        pub fn new(link_capacity: u32, hosts: Vec<Host>) -> Self {
            Simulator {
                link_capacity,
                hosts,
            }
        }

        pub fn advance(&mut self, duration: u32) {
            let speed: u32 = self.hosts.iter().map(|host| host.speed()).sum();
            let lost = speed > self.link_capacity;
            for host in &mut self.hosts {
                host.advance(duration);
                host.adjust(lost);
            }
        }
    }

    impl std::fmt::Debug for Simulator {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            for (i, host) in self.hosts.iter().enumerate() {
                write!(f, "C{}.cwnd = {},", i + 1, host.cwnd)?;
            }
            Ok(())
        }
    }
}

use simulate::{Host, Simulator};

fn problem_3_50() {
    println!("\nP3.50");

    let unit = 50;
    let hosts = vec![Host::new(10, 50), Host::new(10, 100)];
    let mut simulator = Simulator::new(30, hosts);
    for t in (0..1000).step_by(unit) {
        simulator.advance(unit as u32);
        println!("at the end of t = {:>4}ms, {:?}", t + unit, simulator);
    }
}

fn problem_3_51() {
    println!("\nP3.51");

    let unit = 100;
    let hosts = vec![Host::new(15, 100), Host::new(10, 100)];
    let mut simulator = Simulator::new(30, hosts);
    for t in (0..2200).step_by(unit) {
        simulator.advance(unit as u32);
        println!("at the end of t = {:>4}ms, {:?}", t + unit, simulator);
    }
}
