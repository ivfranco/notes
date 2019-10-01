use gbn::{
    channel::Channel,
    protocol::{GBNReceiver, GBNSender, Message},
};
use std::time::Duration;

#[test]
fn tunneling() {
    env_logger::init();

    let channel = Channel::new(0.1, 0.1, Duration::from_millis(100));
    let (sender, send_channel_out) = GBNSender::new(4, Duration::from_millis(1000));
    let (receiver, recv_channel_out, data_out) = GBNReceiver::new();
    let data_in = sender.event_send();

    channel
        .clone()
        .connect(send_channel_out, receiver.event_send());
    channel.connect(recv_channel_out, sender.event_send());
    sender.process();
    receiver.process();

    for i in 0..20u32 {
        data_in
            .send(Message::Data(i.to_be_bytes().to_vec()))
            .unwrap();
    }

    for i in 0..20u32 {
        let out = data_out.recv().unwrap();
        assert_eq!(out, i.to_be_bytes());
    }

    data_in.send(Message::Terminate).unwrap();
}
