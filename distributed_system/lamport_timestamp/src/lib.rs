#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::time::Duration;

use rand::Rng;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ProcessId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Lamport {
    counter: u32,
    id: ProcessId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MessageKind {
    Request,
    Release,
    Acknowledge(ProcessId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Message {
    timestamp: Lamport,
    kind: MessageKind,
}

struct Broadcast {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Broadcast {
    fn new(sender: &Sender<Message>) -> Self {
        Self {
            sender: sender.clone(),
            receiver: sender.subscribe(),
        }
    }

    fn recipients(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Clone for Broadcast {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
        }
    }
}

struct Process {
    /// The unique immutable process id.
    id: ProcessId,
    /// The counter of events.
    counter: u32,
    /// A min heap of request timestamps, the earlist (wrt. to lamport timestamp total order) will be
    /// on top.
    request_queue: BinaryHeap<Reverse<Message>>,
    /// The broadcast channel.
    broadcast: Broadcast,
}

impl Process {
    fn new(id: ProcessId, sender: &Sender<Message>) -> Self {
        Self {
            id,
            counter: 1,
            request_queue: Default::default(),
            broadcast: Broadcast::new(sender),
        }
    }

    async fn event_loop(&mut self) {
        loop {
            let msg = self.broadcast.receiver.recv().await.unwrap();
            self.handle(msg).await;
            if self.granted() {
                self.acquire_resource().await;
            }
        }
    }

    async fn handle(&mut self, msg: Message) {
        // ignore own messages
        if msg.timestamp.id == self.id {
            return;
        }

        match msg.kind {
            MessageKind::Request => {
                // Rule 2: send acknowledge message back
                self.maybe_bump(&msg.timestamp);

                self.send(MessageKind::Acknowledge(msg.timestamp.id));
                self.request_queue.push(Reverse(msg));
            }

            MessageKind::Release => {
                // Rule 4: clear request queue
                self.maybe_bump(&msg.timestamp);

                self.request_queue.clear();

                //
                self.request();
            }

            MessageKind::Acknowledge(id) if id == self.id => {
                self.maybe_bump(&msg.timestamp);

                self.request_queue.push(Reverse(msg));
            }

            _ => (),
        }
    }

    fn maybe_bump(&mut self, timestamp: &Lamport) {
        if timestamp.counter >= self.counter {
            self.counter = timestamp.counter + 1;
        }
    }

    fn granted(&self) -> bool {
        // Rule 5
        // the first request in the queue must be from the current process
        let first_request_timestamp = self.request_queue.iter().find_map(|msg| {
            if msg.0.kind == MessageKind::Request {
                Some(msg.0.timestamp)
            } else {
                None
            }
        });

        let first_request_timestamp = if let Some(t) = first_request_timestamp {
            t
        } else {
            return false;
        };

        if first_request_timestamp.id != self.id {
            return false;
        }

        // Must have received message from all other processes after the timestamp
        let t = first_request_timestamp.counter;
        let heard_recipients = self
            .request_queue
            .iter()
            .filter(|msg| {
                let timestamp = msg.0.timestamp;
                timestamp.counter > t && timestamp.id != self.id
            })
            .map(|msg| msg.0.timestamp.id)
            .collect::<HashSet<_>>();

        heard_recipients.len() + 1 >= self.broadcast.recipients()
    }

    async fn acquire_resource(&mut self) {
        println!("{:?} acquired resource", self.id);
        let duration = Duration::from_millis(rand::thread_rng().gen_range(500..2000));
        tokio::time::sleep(duration).await;
        println!("{:?} released resource", self.id);
        self.release();
    }

    fn release(&mut self) {
        // Rule 3: clear request queue, send release message to all other processes
        self.request_queue.clear();
        self.send(MessageKind::Release);
    }

    fn request(&mut self) {
        // Rule 1: send request message to all other processes with timestamp, push the same
        // messsage to request queue
        self.request_queue
            .push(Reverse(self.message(MessageKind::Request)));
        self.send(MessageKind::Request);
    }

    fn message(&self, kind: MessageKind) -> Message {
        let timestamp = Lamport {
            id: self.id,
            counter: self.counter,
        };

        Message { timestamp, kind }
    }

    fn send(&mut self, kind: MessageKind) {
        let msg = self.message(kind);
        self.counter += rand::thread_rng().gen_range(1..10);

        self.broadcast
            .sender
            .send(msg)
            .expect("All recipient dropped");
    }

    fn clear(&mut self) {
        self.request_queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn lamport_exclusive_resource() {
        let (sender, _) = tokio::sync::broadcast::channel(256);

        for id in 0..8 {
            let mut process = Process::new(ProcessId(id), &sender);
            tokio::spawn(async move {
                process.event_loop().await;
            });
        }

        sender
            .send(Message {
                kind: MessageKind::Release,
                timestamp: Lamport {
                    counter: 0,
                    id: ProcessId(0),
                },
            })
            .unwrap();

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
