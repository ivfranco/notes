use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use actix::{Actor, AsyncContext, Context, Handler, Recipient};
use bitvec::{bitvec, prelude::BitVec};

use crate::{Lamport, Message, MessageKind, Process, ProcessId};

struct MessageHub {
    recipients: HashMap<ProcessId, Recipient<Message>>,
}

impl MessageHub {
    // broadcast the message to all processes except the sender
    fn broadcast(&self, msg: Message) {
        let sender = msg.timestamp.id;
        for (&id, recipient) in &self.recipients {
            if id != sender {
                recipient.do_send(msg.clone()).unwrap();
            }
        }
    }
}

impl Actor for MessageHub {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(256);

        const RECIPIENTS: u32 = 8;

        let hub = ctx.address().recipient();

        for id in 0..RECIPIENTS {
            let pid = ProcessId(id);
            let addr = ProcessActor::new(pid, RECIPIENTS, hub.clone()).start();
            self.recipients.insert(pid, addr.recipient());
        }
    }
}

impl Handler<Message> for MessageHub {
    type Result = ();

    fn handle(&mut self, msg: Message, _ctx: &mut Self::Context) -> Self::Result {
        match msg.kind {
            MessageKind::Request | MessageKind::Release => {
                self.broadcast(msg);
            }
            MessageKind::Acknowledge(id) => {
                // never error: do_send ignores capacity, actors are never terminated
                self.recipients[&id].do_send(msg).unwrap();
            }
        }
    }
}

struct ProcessActor {
    process: Process,
    hub: Recipient<Message>,
}
