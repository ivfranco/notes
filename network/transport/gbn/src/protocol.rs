use std::{
    mem::size_of,
    ops::{Deref, DerefMut},
    sync::mpsc::{channel, Sender, Receiver},
    time::Duration,
    thread,
};
use crate::checksum;

mod consts {
    use super::*;

    pub const SN_LEN: usize = size_of::<u32>();
    pub const CS_LEN: usize = size_of::<u16>();

    pub const SN_OFFSET: usize = 0;
    pub const CS_OFFSET: usize = SN_OFFSET + SN_LEN;
    pub const DATA_OFFSET: usize = CS_OFFSET + CS_LEN;

    pub const MAX_WINDOW_SIZE: u32 = u32::max_value() / 2;
}

use consts::*;

#[derive(Clone)]
pub struct Packet(Vec<u8>);

impl Packet {
    pub fn new(sn: u32, data: &[u8]) -> Self {
        let mut packet = vec![0; DATA_OFFSET];
        packet[SN_OFFSET .. SN_OFFSET + SN_LEN].copy_from_slice(&sn.to_be_bytes());
        packet.extend_from_slice(data);
        let checksum = checksum(&packet);
        packet[CS_OFFSET .. CS_OFFSET + CS_LEN].copy_from_slice(&checksum);

        Self(packet)
    }

    fn new_ack(sn: u32) -> Self {
        Self::new(sn, &[])
    }

    pub fn corrupted(&self) -> bool {
        checksum(&self.0) != [0, 0]
    }

    fn sn(&self) -> u32 {
        let mut bytes = [0; size_of::<u32>()];
        bytes.copy_from_slice(&self.0[SN_OFFSET .. SN_OFFSET + SN_LEN]);
        u32::from_be_bytes(bytes)
    }

    fn data(&self) -> &[u8] {
        &self.0[DATA_OFFSET ..]
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

const INIT_SEQUENTIAL_NUMBER: u32 = 0;

pub struct GBNSender {
    event_recv: Receiver<Message>,
    event_send: Sender<Message>,
    channel: Sender<Packet>,
    window: u32,
    sn: u32,
    timeout: Duration,
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
            window,
            sn: INIT_SEQUENTIAL_NUMBER,
            timeout,
        };

        (sender, channel_output)
    }

    pub fn event_send(&self) -> Sender<Message> {
        self.event_send.clone()
    }

    pub fn process(self) {
        // handle events in another thread
        unimplemented!()
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
                    let sn = if packet.corrupted() || packet.sn() != self.sn {
                        self.sn.wrapping_sub(1) 
                    } else {
                        self.sn
                    };

                    if sn == self.sn {
                        self.sn = self.sn.wrapping_add(1);
                        self.upper.send(packet.data().to_vec()).unwrap();
                    }

                    self.channel.send(Packet::new_ack(sn)).unwrap();
                }
                Message::Terminate => break,
                _ => unreachable!(),
            }
        }
    }
}
