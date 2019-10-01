use crate::checksum;
use log::{error, info};
use std::{
    collections::VecDeque,
    mem::size_of,
    ops::{Deref, DerefMut},
    sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};

mod consts {
    use super::*;

    pub const TYPE_LEN: usize = 2;
    pub const SN_LEN: usize = size_of::<u32>();
    pub const CS_LEN: usize = size_of::<u16>();

    pub const TYPE_OFFSET: usize = 0;
    pub const SN_OFFSET: usize = TYPE_OFFSET + TYPE_LEN;
    // checksum must be aligned to size of u16
    pub const CS_OFFSET: usize = SN_OFFSET + SN_LEN;
    pub const DATA_OFFSET: usize = CS_OFFSET + CS_LEN;

    pub const MAX_WINDOW_SIZE: u32 = u32::max_value() / 2;
    pub const INIT_SEQUENTIAL_NUMBER: u32 = 0;
}

use consts::*;

#[derive(Clone, Copy, PartialEq)]
enum Type {
    Packet = 0,
    ACK = 1,
}

#[derive(Clone, PartialEq)]
pub struct Packet(Vec<u8>);

impl Packet {
    fn new(ty: Type, sn: u32, data: &[u8]) -> Self {
        let mut packet = vec![0; DATA_OFFSET];
        packet[TYPE_OFFSET..TYPE_OFFSET + TYPE_LEN].copy_from_slice(&(ty as u16).to_be_bytes());
        packet[SN_OFFSET..SN_OFFSET + SN_LEN].copy_from_slice(&sn.to_be_bytes());
        packet.extend_from_slice(data);
        let checksum = checksum(&packet);
        packet[CS_OFFSET..CS_OFFSET + CS_LEN].copy_from_slice(&checksum);

        Self(packet)
    }

    pub fn new_packet(sn: u32, data: &[u8]) -> Self {
        Self::new(Type::Packet, sn, data)
    }

    pub fn new_ack(sn: u32) -> Self {
        Self::new(Type::ACK, sn, &[])
    }

    pub fn corrupted(&self) -> bool {
        checksum(&self) != [0, 0]
    }

    pub fn sn(&self) -> u32 {
        let mut bytes = [0; SN_LEN];
        bytes.copy_from_slice(&self.0[SN_OFFSET..SN_OFFSET + SN_LEN]);
        u32::from_be_bytes(bytes)
    }

    pub fn data(&self) -> &[u8] {
        &self[DATA_OFFSET..]
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.corrupted() {
            write!(f, "CORRUPTED")?;
        } else {
            let mut bytes = [0; TYPE_LEN];
            bytes.copy_from_slice(&self[TYPE_OFFSET..TYPE_OFFSET + TYPE_LEN]);
            let ty = u16::from_be_bytes(bytes);
            if ty == Type::Packet as u16 {
                write!(
                    f,
                    "Packet {{ sn = {}, data = {:?} }}",
                    self.sn(),
                    self.data()
                )?;
            } else {
                write!(f, "ACK {{ sn = {} }}", self.sn())?;
            }
        }

        Ok(())
    }
}

impl Deref for Packet {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum Message {
    // Terminates the sender / receiver
    Terminate,
    // Packets from the unreliable channel
    Packet(Packet),
    // Data from application layer
    Data(Vec<u8>),
}

struct Timer {
    deadline: Instant,
    timeout: Duration,
}

impl Timer {
    fn new(timeout: Duration) -> Self {
        Self {
            deadline: Instant::now() + timeout,
            timeout,
        }
    }

    fn until_deadline(&self) -> Duration {
        self.deadline - Instant::now()
    }

    fn restart(&mut self) {
        self.deadline = Instant::now() + self.timeout;
    }
}

struct PacketQueue {
    sn: u32,
    window: u32,
    waiting: u32,
    queue: VecDeque<Packet>,
}

impl PacketQueue {
    fn new(window: u32) -> Self {
        Self {
            sn: INIT_SEQUENTIAL_NUMBER,
            window,
            waiting: 0,
            queue: VecDeque::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.waiting == 0
    }

    fn push(&mut self, data: &[u8]) {
        let next_sn = self.sn.wrapping_add(self.queue.len() as u32);
        self.queue.push_back(Packet::new_packet(next_sn, data));
    }

    fn ack(&mut self, ack_sn: u32) {
        if self.sn != ack_sn && self.sn.wrapping_sub(ack_sn) <= self.window {
            // delayed ack to retransmission
            info!("Duplicated ACK to packet {}", ack_sn);
        } else {
            for _ in 0..=ack_sn.wrapping_sub(self.sn) {
                info!("ACKed packet {}", self.sn);
                self.queue.pop_front().expect("PacketQueue::ack");
                self.sn = self.sn.wrapping_add(1);
                self.waiting -= 1;
            }
        }
    }

    fn flush(&mut self, channel: &Sender<Packet>) {
        while self.waiting < self.window {
            if let Some(packet) = self.queue.get(self.waiting as usize) {
                channel.send(packet.clone()).unwrap_or_else(|_| {
                    error!("PacketQueue::flush: Receiver dropped");
                });
                info!("Send packet: {:?}", packet);
                self.waiting += 1;
            } else {
                break;
            }
        }
    }

    fn retransmit(&self, channel: &Sender<Packet>) {
        for i in 0..self.waiting as usize {
            info!("Retransmitting packet {}", self.sn.wrapping_add(i as u32));
            channel.send(self.queue[i].clone()).unwrap_or_else(|_| {
                error!("PacketQueue::retransmit: Receiver dropped");
            })
        }
    }
}

pub struct GBNSender {
    event_recv: Receiver<Message>,
    event_send: Sender<Message>,
    channel: Sender<Packet>,
    timer: Timer,
    queue: PacketQueue,
}

impl GBNSender {
    pub fn new(window: u32, timeout: Duration) -> (Self, Receiver<Packet>) {
        assert!(window <= MAX_WINDOW_SIZE);

        let (event_send, event_recv) = channel();
        let (channel_input, channel_output) = channel();

        let sender = Self {
            event_recv,
            event_send,
            channel: channel_input,
            timer: Timer::new(timeout),
            queue: PacketQueue::new(window),
        };

        (sender, channel_output)
    }

    pub fn event_send(&self) -> Sender<Message> {
        self.event_send.clone()
    }

    pub fn process(mut self) {
        thread::spawn(move || {
            self.handle();
        });
    }

    fn until_deadline(&self) -> Duration {
        self.timer.until_deadline()
    }

    fn timer_restart(&mut self) {
        self.timer.restart();
    }

    fn ack(&mut self, ack_sn: u32) {
        self.queue.ack(ack_sn);
    }

    fn retransmit(&self) {
        self.queue.retransmit(&self.channel);
    }

    fn flush(&mut self) {
        self.queue.flush(&self.channel);
    }

    fn handle(&mut self) {
        use Message::*;
        use RecvTimeoutError::*;

        loop {
            match self.event_recv.recv_timeout(self.until_deadline()) {
                Ok(Data(data)) => {
                    info!("Received data from upper layer: {:?}", data);
                    self.queue.push(&data);
                }
                Ok(Packet(ref packet)) if !packet.corrupted() => {
                    info!("Sender received packet: {:?}", packet);
                    self.ack(packet.sn());
                    if self.queue.is_empty() {
                        info!("Sender buffer emptied, restart timer");
                        self.timer_restart();
                    }
                }
                Ok(Terminate) | Err(Disconnected) => {
                    info!("Channel disconnected / terminate signal received, tear down sender");
                    break;
                }
                Err(Timeout) => {
                    info!("Sender timed out, retransmitting unacknowledged packets");
                    self.retransmit();
                    info!("Retransmission done, restart sender timer");
                    self.timer_restart();
                }
                _ => (),
            }
            self.flush();
        }
    }
}

pub struct GBNReceiver {
    event_recv: Receiver<Message>,
    event_send: Sender<Message>,
    channel: Sender<Packet>,
    upper: Sender<Vec<u8>>,
    sn: u32,
}

impl GBNReceiver {
    pub fn new() -> (Self, Receiver<Packet>, Receiver<Vec<u8>>) {
        let (event_send, event_recv) = channel();
        let (channel_input, channel_output) = channel();
        let (upper_input, upper_output) = channel();

        let receiver = Self {
            event_recv,
            event_send,
            channel: channel_input,
            upper: upper_input,
            sn: INIT_SEQUENTIAL_NUMBER,
        };

        (receiver, channel_output, upper_output)
    }

    pub fn event_send(&self) -> Sender<Message> {
        self.event_send.clone()
    }

    pub fn process(mut self) {
        thread::spawn(move || self.handle());
    }

    fn handle(&mut self) {
        for event in &self.event_recv {
            match event {
                Message::Packet(packet) => {
                    info!("Receiver received packet: {:?}", packet);
                    let sn = if packet.corrupted() || packet.sn() != self.sn {
                        self.sn.wrapping_sub(1)
                    } else {
                        self.sn
                    };

                    if sn == self.sn {
                        self.sn = self.sn.wrapping_add(1);
                        self.upper.send(packet.data().to_vec()).unwrap_or_else(|_| {
                            error!("GBNReceiver::handle: receiver dropped");
                        });
                    }

                    info!("Receiver send ACK {}", sn);
                    self.channel.send(Packet::new_ack(sn)).unwrap_or_else(|_| {
                        error!("GBNReceiver::handle: receiver dropped");
                    });
                }
                Message::Terminate => {
                    info!("Terminate signal received, tear down receiver");
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn sender_test() {
    let (sender, output) = GBNSender::new(4, Duration::from_millis(500));
    let events = sender.event_send();

    sender.process();

    for _ in 0..5 {
        events.send(Message::Data(vec![])).unwrap();
    }

    for sn in 0..4 {
        let packet = output.recv().expect("First four original packets");
        assert_eq!(packet.sn(), sn);
    }
    assert_eq!(
        output.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout)
    );

    thread::sleep(Duration::from_millis(500));

    for sn in 0..4 {
        let packet = output.recv().expect("Four retransmission packets");
        assert_eq!(packet.sn(), sn);
    }
    assert_eq!(
        output.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout)
    );
}
