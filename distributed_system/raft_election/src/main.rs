use std::fmt::Debug;
use std::time::Duration;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, SpawnHandle, System};
use log::{info, warn};
use rand::Rng;

const SLOW_MOTION: u64 = 20;
const MIN_ELECTION_TIMEOUT: u64 = 150 * SLOW_MOTION;
const MAX_ELECTION_TIMEOUT: u64 = 300 * SLOW_MOTION;
const HEARTBEAT_INTERVAL: u64 = 100 * SLOW_MOTION;

fn main() {
    env_logger::init();

    System::new().block_on(async {
        let hub = MessageHub::new(5);
        hub.start();
        actix::clock::sleep(Duration::from_secs(1000)).await;
    });
}

type NodeId = usize;

struct MessageHub {
    peer_count: usize,
    peers: Vec<Addr<Raft>>,
}

impl MessageHub {
    fn new(peer_count: usize) -> Self {
        Self {
            peer_count,
            peers: Vec::new(),
        }
    }
}

impl Actor for MessageHub {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        for id in 0..self.peer_count {
            let raft = Raft::new(id, self.peer_count, ctx.address());
            self.peers.push(raft.start());
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct To<M>(M, NodeId);

impl<M> Handler<To<M>> for MessageHub
where
    M: Message + Debug + Send + 'static,
    M::Result: Send,
    Raft: Handler<M>,
{
    type Result = ();

    fn handle(&mut self, msg: To<M>, ctx: &mut Self::Context) -> Self::Result {
        let To(msg, peer) = msg;
        let mut rng = rand::thread_rng();
        let ms = rng.gen_range(25..75) * SLOW_MOTION;

        if rng.gen_bool(0.90) {
            ctx.run_later(Duration::from_millis(ms), move |hub, _ctx| {
                hub.peers[peer].do_send(msg);
            });
        } else {
            warn!("message {:?} to node {} was lost", msg, peer);
        }
    }
}

enum Role {
    Follower,
    Candidate(Vec<bool>),
    Leader,
}

struct Raft {
    // configurations and references
    me: NodeId,
    peer_count: usize,
    hub: Addr<MessageHub>,
    timeout: Option<SpawnHandle>,

    // persistent states
    current_term: u64,
    voted_for: Option<NodeId>,

    // volatile states
    role: Role,
}

impl Raft {
    fn new(id: NodeId, peer_count: usize, hub: Addr<MessageHub>) -> Self {
        Self {
            me: id,
            peer_count,
            hub,
            timeout: None,
            current_term: 0,
            voted_for: None,
            role: Role::Follower,
        }
    }

    fn cancel_timeout(&mut self, ctx: &mut <Self as Actor>::Context) {
        if let Some(handle) = self.timeout.take() {
            ctx.cancel_future(handle);
        }
    }

    fn reset_timeout(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.cancel_timeout(ctx);

        let duration = Duration::from_millis(
            rand::thread_rng().gen_range(MIN_ELECTION_TIMEOUT..=MAX_ELECTION_TIMEOUT),
        );
        self.timeout = Some(ctx.run_later(duration, |raft, ctx| {
            raft.timeout(ctx);
        }))
    }

    fn timeout(&mut self, ctx: &mut <Self as Actor>::Context) {
        match self.role {
            Role::Follower | Role::Candidate(..) => {
                self.start_election(ctx);
            }
            Role::Leader => {
                unreachable!("Should have been cancelled during transition");
            }
        }

        self.reset_timeout(ctx)
    }

    fn update_term(&mut self, term: u64) {
        if term > self.current_term {
            self.current_term = term;
            self.role = Role::Follower;
            self.voted_for = None;
        }
    }

    fn pre_message(&mut self, term: u64, ctx: &mut <Self as Actor>::Context) {
        self.update_term(term);
        if !matches!(self.role, Role::Leader) {
            self.reset_timeout(ctx);
        }
    }

    fn start_election(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.timeout.take();
        self.current_term += 1;

        info!(
            "Node {} started election of term {}",
            self.me, self.current_term
        );

        let mut votes = vec![false; self.peer_count];
        votes[self.me] = true;
        self.voted_for = Some(self.me);
        self.role = Role::Candidate(votes);

        let request_vote = RequestVoteArgs {
            term: self.current_term,
            candidate_id: self.me,
        };

        for peer in (0..self.peer_count).filter(|id| *id != self.me) {
            self.hub.do_send(To(request_vote.clone(), peer));
        }
    }

    fn start_leadership(&mut self, ctx: &mut <Self as Actor>::Context) {
        info!(
            "Node {} became the leader of term {}",
            self.me, self.current_term
        );

        self.cancel_timeout(ctx);
        self.role = Role::Leader;

        self.heartbeat();

        let handle = ctx.run_interval(Duration::from_millis(HEARTBEAT_INTERVAL), |raft, _ctx| {
            raft.heartbeat();
        });

        self.timeout = Some(handle);
    }

    fn heartbeat(&self) {
        for peer in (0..self.peer_count).filter(|id| *id != self.me) {
            self.hub.do_send(To(
                AppendEntriesArgs {
                    term: self.current_term,
                },
                peer,
            ));
        }
    }
}

impl Actor for Raft {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.reset_timeout(ctx);
    }
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
struct RequestVoteArgs {
    term: u64,
    candidate_id: NodeId,
}

impl Handler<RequestVoteArgs> for Raft {
    type Result = ();

    fn handle(&mut self, msg: RequestVoteArgs, ctx: &mut Self::Context) -> Self::Result {
        self.pre_message(msg.term, ctx);

        let vote_granted = self.current_term <= msg.term
            && (self.voted_for.is_none() || self.voted_for == Some(msg.candidate_id));

        if vote_granted {
            info!("Node {} voted for node {}", self.me, msg.candidate_id);

            self.voted_for = Some(msg.candidate_id);
            self.role = Role::Follower;
        }

        self.hub.do_send(To(
            RequestVoteReply {
                from: self.me,
                term: self.current_term,
                vote_granted,
            },
            msg.candidate_id,
        ));
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
struct RequestVoteReply {
    from: NodeId,
    term: u64,
    vote_granted: bool,
}

impl Handler<RequestVoteReply> for Raft {
    type Result = ();

    fn handle(&mut self, msg: RequestVoteReply, ctx: &mut Self::Context) -> Self::Result {
        self.pre_message(msg.term, ctx);

        if let Role::Candidate(votes) = &mut self.role {
            if msg.vote_granted {
                votes[msg.from] = true;
            }

            if votes.iter().filter(|b| **b).count() > self.peer_count / 2 {
                self.start_leadership(ctx);
            }
        }
    }
}

#[derive(Debug, Message)]
#[rtype(results = "()")]
struct AppendEntriesArgs {
    term: u64,
}

impl Handler<AppendEntriesArgs> for Raft {
    type Result = ();

    fn handle(&mut self, msg: AppendEntriesArgs, ctx: &mut Self::Context) -> Self::Result {
        self.pre_message(msg.term, ctx);

        if msg.term == self.current_term {
            match self.role {
                Role::Follower => (),
                Role::Candidate(..) => {
                    self.role = Role::Follower;
                }
                Role::Leader => {
                    unreachable!("Multiple leader in the same term");
                }
            }
        }
    }
}
