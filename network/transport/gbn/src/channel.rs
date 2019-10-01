use crate::protocol::{Message, Packet};
use log::{error, info, warn};
use rand::{thread_rng, Rng};
use std::{
    collections::VecDeque,
    sync::mpsc::{Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};

/// An unreliable and unbounded channel with infinite buffer.\
/// Will corrupt and loss packets, but won't reorder them.
#[derive(Clone)]
pub struct Channel {
    loss_rate: f64,
    corrupt_rate: f64,
    rtt: Duration,
    queue: VecDeque<(Packet, Instant)>,
}

impl Channel {
    pub fn new(loss_rate: f64, corrupt_rate: f64, rtt: Duration) -> Self {
        assert!(0.0 <= loss_rate && loss_rate <= 1.0);
        assert!(0.0 <= corrupt_rate && corrupt_rate <= 1.0);

        Channel {
            loss_rate,
            corrupt_rate,
            rtt,
            queue: VecDeque::new(),
        }
    }

    fn next_deadline(&self) -> Option<Instant> {
        self.queue.front().map(|(_, t)| *t)
    }

    fn next_duration(&self) -> Duration {
        self.next_deadline()
            .map(|t| {
                let now = Instant::now();
                if t < now {
                    Duration::from_secs(0)
                } else {
                    t - now
                }
            })
            .unwrap_or(FOREVER)
    }

    fn pop_packets(&mut self, to: &Sender<Message>) {
        let now = Instant::now();
        while let Some((packet, deadline)) = self.queue.pop_front() {
            if deadline <= now {
                to.send(Message::Packet(packet)).unwrap_or_else(|_| {
                    error!("Channel::pop_packets: receiver dropped");
                })
            } else {
                self.queue.push_front((packet, deadline));
                break;
            }
        }
    }

    pub fn connect(mut self, from: Receiver<Packet>, to: Sender<Message>) {
        use RecvTimeoutError::*;

        thread::spawn(move || {
            let mut rng = thread_rng();
            loop {
                match from.recv_timeout(self.next_duration()) {
                    Ok(mut packet) => {
                        if self.loss_rate >= rng.gen() {
                            info!("Channel lost packet: {:?}", packet);
                            continue;
                        }
                        if self.corrupt_rate >= rng.gen() {
                            info!("Channel corrupted packet: {:?}", packet);
                            corrupt(&mut packet);
                        }
                        // one way delay, half the rtt
                        self.queue
                            .push_back((packet, Instant::now() + self.rtt / 2));
                    }
                    Err(Timeout) => self.pop_packets(&to),
                    Err(Disconnected) => {
                        info!("Channel disconnected");
                        to.send(Message::Terminate).unwrap_or_else(|_| {
                            warn!("Send terminate but the receiver was already dropped");
                        });
                        break;
                    }
                }
            }
        });
    }
}

const FOREVER: Duration = Duration::from_secs(100_000_000_000);

/// Introduce one byte error to a packet.
fn corrupt(packet: &mut Packet) {
    let mut rng = thread_rng();
    let len = packet.len();
    let idx = rng.gen_range(0, len);
    packet[idx] = !packet[idx];
}

#[test]
fn corrupt_test() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let sn = rng.gen();
        let mut data = vec![0; rng.gen_range(10, 100)];
        rng.fill(data.as_mut_slice());

        let mut packet = Packet::new_packet(sn, &data);
        assert!(!packet.corrupted());
        corrupt(&mut packet);
        // one byte error should always be detectable
        assert!(packet.corrupted());
    }
}
