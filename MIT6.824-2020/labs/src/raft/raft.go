package raft

//
// this is an outline of the API that raft must expose to
// the service (or tester). see comments below for
// each of these functions for more details.
//
// rf = Make(...)
//   create a new Raft server.
// rf.Start(command interface{}) (index, term, isleader)
//   start agreement on a new log entry
// rf.GetState() (term, isLeader)
//   ask a Raft for its current term, and whether it thinks it is leader
// ApplyMsg
//   each time a new entry is committed to the log, each Raft peer
//   should send an ApplyMsg to the service (or tester)
//   in the same server.
//

import (
	//	"bytes"

	"log"
	"math/rand"
	"sync"
	"sync/atomic"
	"time"

	//	"6.824/labgob"
	"6.824/labrpc"
)

const (
	HEARTBEAT_INTERVAL   time.Duration = time.Millisecond * 200
	MIN_ELECTION_TIMEOUT time.Duration = time.Millisecond * 500
	MAX_ELECTION_TIMEOUT time.Duration = time.Millisecond * 1000

	// Valid server id starts from 0
	NOBODY int = -1
)

func randomElectionTimeout() time.Duration {
	r := rand.Int63n(MAX_ELECTION_TIMEOUT.Nanoseconds() - MIN_ELECTION_TIMEOUT.Nanoseconds())
	return MIN_ELECTION_TIMEOUT + time.Duration(r)
}

type Role int

const (
	Leader Role = iota
	Candidate
	Follower
)

// as each Raft peer becomes aware that successive log entries are
// committed, the peer should send an ApplyMsg to the service (or
// tester) on the same server, via the applyCh passed to Make(). set
// CommandValid to true to indicate that the ApplyMsg contains a newly
// committed log entry.
//
// in part 2D you'll want to send other kinds of messages (e.g.,
// snapshots) on the applyCh, but set CommandValid to false for these
// other uses.
type ApplyMsg struct {
	CommandValid bool
	Command      interface{}
	CommandIndex int

	// For 2D:
	SnapshotValid bool
	Snapshot      []byte
	SnapshotTerm  int
	SnapshotIndex int
}

// A Go object implementing a single Raft peer.
type Raft struct {
	mu        sync.Mutex          // Lock to protect shared access to this peer's state
	peers     []*labrpc.ClientEnd // RPC end points of all peers
	persister *Persister          // Object to hold this peer's persisted state
	me        int                 // this peer's index into peers[]
	dead      int32               // set by Kill()

	// Your data here (2A, 2B, 2C).
	// Look at the paper's Figure 2 for a description of what
	// state a Raft server must maintain.

	// Should be persisted
	currentTerm int
	votedFor    int

	// Volatile
	role      Role
	cancelled *bool
}

// return currentTerm and whether this server
// believes it is the leader.
func (rf *Raft) GetState() (int, bool) {
	// Your code here (2A).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	term := rf.currentTerm
	isleader := rf.role == Leader
	return term, isleader
}

// save Raft's persistent state to stable storage,
// where it can later be retrieved after a crash and restart.
// see paper's Figure 2 for a description of what should be persistent.
func (rf *Raft) persist() {
	// Your code here (2C).
	// Example:
	// w := new(bytes.Buffer)
	// e := labgob.NewEncoder(w)
	// e.Encode(rf.xxx)
	// e.Encode(rf.yyy)
	// data := w.Bytes()
	// rf.persister.SaveRaftState(data)
}

// restore previously persisted state.
func (rf *Raft) readPersist(data []byte) {
	if data == nil || len(data) < 1 { // bootstrap without any state?
		return
	}
	// Your code here (2C).
	// Example:
	// r := bytes.NewBuffer(data)
	// d := labgob.NewDecoder(r)
	// var xxx
	// var yyy
	// if d.Decode(&xxx) != nil ||
	//    d.Decode(&yyy) != nil {
	//   error...
	// } else {
	//   rf.xxx = xxx
	//   rf.yyy = yyy
	// }
}

// A service wants to switch to snapshot.  Only do so if Raft hasn't
// have more recent info since it communicate the snapshot on applyCh.
func (rf *Raft) CondInstallSnapshot(lastIncludedTerm int, lastIncludedIndex int, snapshot []byte) bool {

	// Your code here (2D).

	return true
}

// the service says it has created a snapshot that has
// all info up to and including index. this means the
// service no longer needs the log through (and including)
// that index. Raft should now trim its log as much as possible.
func (rf *Raft) Snapshot(index int, snapshot []byte) {
	// Your code here (2D).

}

func (rf *Raft) updateCurrentTerm(term int) (reset bool) {
	if rf.currentTerm < term {
		rf.currentTerm = term
		rf.role = Follower
		rf.votedFor = NOBODY

		return true
	}

	return false
}

func (rf *Raft) cancelTimeout() {
	*rf.cancelled = true
	cancelled := false
	rf.cancelled = &cancelled
}

func (rf *Raft) setFollowerTimeout() {
	cancelled := rf.cancelled
	go rf.followerTimeout(cancelled)
}

func (rf *Raft) setCandidateTimeout() {
	cancelled := rf.cancelled
	go rf.candidateTimeout(cancelled)
}

func (rf *Raft) setHeartbeat() {
	cancelled := rf.cancelled
	go rf.heartbeat(cancelled)
}

// example RequestVote RPC arguments structure.
// field names must start with capital letters!
type RequestVoteArgs struct {
	// Your data here (2A, 2B).
	Term        int
	CandidateId int
}

// example RequestVote RPC reply structure.
// field names must start with capital letters!
type RequestVoteReply struct {
	// Your data here (2A).
	Term        int
	VoteGranted bool
}

// example RequestVote RPC handler.
func (rf *Raft) RequestVote(args *RequestVoteArgs, reply *RequestVoteReply) {
	// Your code here (2A, 2B).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	log.Printf("[%v] Recv RequestVote { Term: %v, CandidateId: %v }", rf.me, args.Term, args.CandidateId)

	reset := rf.updateCurrentTerm(args.Term)
	reply.Term = rf.currentTerm
	if rf.currentTerm > args.Term {
		reply.VoteGranted = false
		return
	}

	if rf.role == Follower && (rf.votedFor == NOBODY || rf.votedFor == args.CandidateId) {
		rf.votedFor = args.CandidateId
		reply.VoteGranted = true
		reset = true
	} else {
		reply.VoteGranted = false
	}

	if reset {
		rf.cancelTimeout()
		rf.setFollowerTimeout()
	}

	log.Printf("[%v] Repl RequestVote { Term: %v, VoteGranted: %v }", rf.me, reply.Term, reply.VoteGranted)
}

// example code to send a RequestVote RPC to a server.
// server is the index of the target server in rf.peers[].
// expects RPC arguments in args.
// fills in *reply with RPC reply, so caller should
// pass &reply.
// the types of the args and reply passed to Call() must be
// the same as the types of the arguments declared in the
// handler function (including whether they are pointers).
//
// The labrpc package simulates a lossy network, in which servers
// may be unreachable, and in which requests and replies may be lost.
// Call() sends a request and waits for a reply. If a reply arrives
// within a timeout interval, Call() returns true; otherwise
// Call() returns false. Thus Call() may not return for a while.
// A false return can be caused by a dead server, a live server that
// can't be reached, a lost request, or a lost reply.
//
// Call() is guaranteed to return (perhaps after a delay) *except* if the
// handler function on the server side does not return.  Thus there
// is no need to implement your own timeouts around Call().
//
// look at the comments in ../labrpc/labrpc.go for more details.
//
// if you're having trouble getting RPC to work, check that you've
// capitalized all field names in structs passed over RPC, and
// that the caller passes the address of the reply struct with &, not
// the struct itself.
func (rf *Raft) sendRequestVote(server int, args *RequestVoteArgs, reply *RequestVoteReply) bool {
	ok := rf.peers[server].Call("Raft.RequestVote", args, reply)
	return ok
}

type AppendEntriesArgs struct {
	Term     int
	LeaderId int
}

type AppendEntriesReply struct {
	Term    int
	Success bool
}

func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	log.Printf("[%v] Recv AppendEntries { Term: %v, LeaderId: %v }", rf.me, args.Term, args.LeaderId)

	reset := false
	// the difference to a greater term is it's still the same term, votedFor should not be reset
	if rf.currentTerm == args.Term {
		rf.role = Follower
		reset = true
	}
	reset = reset || rf.updateCurrentTerm(args.Term)

	*reply = AppendEntriesReply{
		Term:    rf.currentTerm,
		Success: rf.currentTerm <= args.Term,
	}

	if reset {
		rf.cancelTimeout()
		rf.setFollowerTimeout()
	}

	log.Printf("[%v] Repl AppendEntries { Term: %v, Success: %v }", rf.me, reply.Term, reply.Success)
}

func (rf *Raft) sendAppendEntry(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	return rf.peers[server].Call("Raft.AppendEntries", args, reply)
}

// the service using Raft (e.g. a k/v server) wants to start
// agreement on the next command to be appended to Raft's log. if this
// server isn't the leader, returns false. otherwise start the
// agreement and return immediately. there is no guarantee that this
// command will ever be committed to the Raft log, since the leader
// may fail or lose an election. even if the Raft instance has been killed,
// this function should return gracefully.
//
// the first return value is the index that the command will appear at
// if it's ever committed. the second return value is the current
// term. the third return value is true if this server believes it is
// the leader.
func (rf *Raft) Start(command interface{}) (int, int, bool) {
	index := -1
	term := -1
	isLeader := true

	// Your code here (2B).

	return index, term, isLeader
}

// the tester doesn't halt goroutines created by Raft after each test,
// but it does call the Kill() method. your code can use killed() to
// check whether Kill() has been called. the use of atomic avoids the
// need for a lock.
//
// the issue is that long-running goroutines use memory and may chew
// up CPU time, perhaps causing later tests to fail and generating
// confusing debug output. any goroutine with a long-running loop
// should call killed() to check whether it should stop.
func (rf *Raft) Kill() {
	atomic.StoreInt32(&rf.dead, 1)
	// Your code here, if desired.
	rf.mu.Lock()
	defer rf.mu.Unlock()

	rf.cancelTimeout()
}

func (rf *Raft) killed() bool {
	z := atomic.LoadInt32(&rf.dead)
	return z == 1
}

// Access to `cancelled` must be protected by the main mutex.
func (rf *Raft) heartbeat(cancelled *bool) {
	for !rf.killed() {
		rf.mu.Lock()

		if *cancelled {
			rf.mu.Unlock()
			break
		}

		args := AppendEntriesArgs{
			Term:     rf.currentTerm,
			LeaderId: rf.me,
		}

		for server := range rf.peers {
			if server != rf.me {
				go func(server int) {
					reply := AppendEntriesReply{}
					if rf.sendAppendEntry(server, &args, &reply) {
						rf.handleAppendEntriesReply(&reply)
					}
				}(server)
			}
		}

		rf.mu.Unlock()
		time.Sleep(HEARTBEAT_INTERVAL)
	}
}

func (rf *Raft) handleAppendEntriesReply(reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.updateCurrentTerm(reply.Term) {
		rf.cancelTimeout()
		rf.setFollowerTimeout()
	}
}

// Thread safety: Safe
// Access to `cancelled` must be protected by the main mutex.
func (rf *Raft) followerTimeout(cancelled *bool) {
	time.Sleep(randomElectionTimeout())

	rf.mu.Lock()
	defer rf.mu.Unlock()

	if *cancelled {
		return
	}

	rf.startElection()
}

// Thread safety: Unsafe
// Access to `cancelled` must be protected by the main mutex.
func (rf *Raft) startElection() {
	rf.currentTerm += 1
	rf.role = Candidate
	rf.cancelTimeout()
	rf.setCandidateTimeout()

	// the same cancelled passed to rf.candidateTimeout
	cancelled := rf.cancelled
	votes := make(map[int]bool)
	votes[rf.me] = true

	args := RequestVoteArgs{
		Term:        rf.currentTerm,
		CandidateId: rf.me,
	}

	for server := range rf.peers {
		if server != rf.me {
			go func(server int) {
				reply := RequestVoteReply{}
				if rf.sendRequestVote(server, &args, &reply) {
					rf.handleRequestVoteReply(cancelled, votes, server, &reply)
				}
			}(server)
		}
	}
}

// Thread safety: Safe
// Access to `cancelled` must be protected by the main mutex.
func (rf *Raft) candidateTimeout(cancelled *bool) {
	time.Sleep(randomElectionTimeout())

	rf.mu.Lock()
	defer rf.mu.Unlock()

	if *cancelled {
		return
	}

	// cancel vote handlers for the current term
	*cancelled = true
	rf.startElection()
}

func (rf *Raft) handleRequestVoteReply(
	cancelled *bool,
	votes map[int]bool,
	server int,
	reply *RequestVoteReply,
) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.updateCurrentTerm(reply.Term) {
		rf.cancelTimeout()
		rf.setFollowerTimeout()
		return
	}

	if *cancelled {
		return
	}

	votes[server] = reply.VoteGranted
	total := len(rf.peers)
	voted := len(votes)
	granted := 0
	for _, v := range votes {
		if v {
			granted += 1
		}
	}

	if granted > total/2 {
		// won the election
		// cancel other vote handlers
		log.Printf("[%v] Won election with %v/%v votes", rf.me, granted, total)
		*cancelled = true
		rf.startLeadership()
	} else if voted-granted > total/2 {
		// lost the election
		// cancel other vote handlers
		*cancelled = true
		// wait for candidate timeout
	}
}

// Thread safety: Unsafe
func (rf *Raft) startLeadership() {
	rf.role = Leader
	rf.cancelTimeout()
	rf.setHeartbeat()
}

// the service or tester wants to create a Raft server. the ports
// of all the Raft servers (including this one) are in peers[]. this
// server's port is peers[me]. all the servers' peers[] arrays
// have the same order. persister is a place for this server to
// save its persistent state, and also initially holds the most
// recent saved state, if any. applyCh is a channel on which the
// tester or service expects Raft to send ApplyMsg messages.
// Make() must return quickly, so it should start goroutines
// for any long-running work.
func Make(
	peers []*labrpc.ClientEnd,
	me int,
	persister *Persister,
	applyCh chan ApplyMsg,
) *Raft {
	rf := &Raft{}
	rf.peers = peers
	rf.persister = persister
	rf.me = me

	// Your initialization code here (2A, 2B, 2C).
	rf.currentTerm = 0
	rf.role = Follower
	rf.votedFor = NOBODY
	cancelled := false
	rf.cancelled = &cancelled

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())

	// start ticker goroutine to start elections
	go rf.followerTimeout(rf.cancelled)

	return rf
}
