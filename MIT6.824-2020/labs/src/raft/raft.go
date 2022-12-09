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

	"fmt"
	"log"
	"math/rand"
	"os"
	"sort"
	"strconv"
	"strings"
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

func min(a int, b int) int {
	if a > b {
		return b
	} else {
		return a
	}
}

func max(a int, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

func randomElectionTimeout() time.Duration {
	r := rand.Int63n(MAX_ELECTION_TIMEOUT.Nanoseconds() - MIN_ELECTION_TIMEOUT.Nanoseconds())
	return MIN_ELECTION_TIMEOUT + time.Duration(r)
}

func moreUpToDate(t0, i0, t1, i1 int) bool {
	return t0 > t1 || (t0 == t1 && i0 > i1)
}

type Role int

const (
	Leader Role = iota
	Candidate
	Follower
)

func (r Role) String() string {
	switch r {
	case Leader:
		return "L"
	case Candidate:
		return "C"
	case Follower:
		return "F"
	default:
		logger.FatalfLn("Impossible value for Role %v", r)
		return ""
	}
}

type LogLevel int

const (
	LogTrace LogLevel = iota
	LogDebug
	LogInfo
	LogWarn
	LogError
)

type Logger struct {
	level LogLevel
}

func MakeFromEnv() Logger {
	var level LogLevel

	switch strings.ToLower(os.Getenv("LOG_LEVEL")) {
	case "trace":
		level = LogTrace
	case "debug":
		level = LogDebug
	case "info":
		level = LogInfo
	case "error":
		level = LogError
	default:
		level = LogWarn
	}

	return Logger{level: level}
}

var logger Logger = MakeFromEnv()

func (l *Logger) PrintfLn(level LogLevel, fmt string, args ...interface{}) {
	if level >= l.level {
		var levelStr string
		switch level {
		case LogTrace:
			levelStr = "Trace"
		case LogDebug:
			levelStr = "Debug"
		case LogInfo:
			levelStr = "Info"
		case LogWarn:
			levelStr = "Warn"
		case LogError:
			levelStr = "Error"
		}

		log.Printf("["+levelStr+"]"+fmt+"\n", args...)
	}
}

func (l *Logger) FatalfLn(fmt string, args ...interface{}) {
	l.PrintfLn(LogError, fmt, args)
	os.Exit(1)
}

type Context atomic.Bool

func NewCancellable() *Context {
	return &Context{}
}

func (c *Context) IsCancelled() bool {
	return (*atomic.Bool)(c).Load()
}

func (c *Context) Cancel() {
	(*atomic.Bool)(c).Store(true)
}

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

type LogEntry struct {
	Command interface{}
	Term    int
}

// A log of commands, possibly with a prefix replaced by a snapshot.
type Logs struct {
	snapshot          []byte
	lastIncludedIndex int
	lastIncludedTerm  int
	liveLogs          []LogEntry
}

func (l *Logs) String() string {
	terms := make([]string, 0, len(l.liveLogs))
	for _, entry := range l.liveLogs {
		terms = append(terms, strconv.Itoa(entry.Term))
	}
	return "(" + strings.Join(terms, "|") + ")"
}

func MakeLogs() Logs {
	return Logs{
		snapshot:          nil,
		lastIncludedIndex: 0,
		lastIncludedTerm:  0,
		liveLogs:          make([]LogEntry, 0),
	}
}

func (l *Logs) lastIndex() int {
	return l.lastIncludedIndex + len(l.liveLogs)
}

func (l *Logs) lastTerm() int {
	return l.termOf(l.lastIndex())
}

func (l *Logs) append(command interface{}, term int) int {
	l.liveLogs = append(l.liveLogs, LogEntry{Command: command, Term: term})
	return l.lastIndex()
}

func (l *Logs) isLive(nextIndex int) bool {
	return nextIndex > l.lastIncludedIndex
}

func (l *Logs) translateIndex(index int) int {
	return index - l.lastIncludedIndex - 1
}

func (l *Logs) get(index int) *LogEntry {
	return &l.liveLogs[l.translateIndex(index)]
}

func (l *Logs) termOf(index int) int {
	if l.isLive(index) {
		return l.get(index).Term
	} else if index == l.lastIncludedIndex {
		return l.lastIncludedTerm
	} else {
		logger.PrintfLn(LogWarn, "Querying term of log entry %v replaced by snapshot", index)
		return -1
	}
}

func (l *Logs) entriesStartFrom(index int) []LogEntry {
	return l.liveLogs[l.translateIndex(index):]
}

func (l *Logs) update(prevIndex int, entries []LogEntry) {
	for i, entry := range entries {
		j := prevIndex + i + 1
		// skip snapshot
		if !l.isLive(j) {
			continue
		}

		k := l.translateIndex(j)
		if k+1 > len(l.liveLogs) {
			l.liveLogs = append(l.liveLogs, entry)
		} else if l.liveLogs[k].Term != entry.Term {
			l.liveLogs = l.liveLogs[:k]
			l.liveLogs = append(l.liveLogs, entry)
		}
	}
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

	// Non-volatile, persist on modification
	logs        Logs
	currentTerm int
	votedFor    int

	// Volatile
	role        Role
	applyCh     chan ApplyMsg
	commitIndex int
	lastApplied int
	ctx         *Context

	// Volatile, Leader only
	nextIndex  []int
	matchIndex []int
}

// return currentTerm and whether this server
// believes it is the leader.
func (rf *Raft) GetState() (int, bool) {
	// Your code here (2A).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	term := rf.currentTerm
	isLeader := rf.role == Leader
	return term, isLeader
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

// Thread safety: Unsafe
func (rf *Raft) updateCurrentTerm(term int) (reset bool) {
	if rf.currentTerm < term {
		rf.currentTerm = term
		rf.role = Follower
		rf.votedFor = NOBODY

		return true
	}

	return false
}

func (rf *Raft) electionTimeout(ctx *Context) {
	time.Sleep(randomElectionTimeout())
	rf.mu.Lock()
	if !ctx.IsCancelled() && rf.role != Leader {
		rf.startElection()
		go rf.electionTimeout(rf.ctx)
	}
	rf.mu.Unlock()
}

func (rf *Raft) heartbeatTicker(ctx *Context) {
	for !rf.killed() {
		rf.mu.Lock()
		if !ctx.IsCancelled() && rf.role == Leader {
			rf.heartbeat()
		}
		rf.mu.Unlock()
		time.Sleep(HEARTBEAT_INTERVAL)
	}
}

// Thread safety: Unsafe
func (rf *Raft) leaderUpdateCommitted() {
	if rf.role != Leader {
		logger.FatalfLn("Raft.leaderUpdateCommitted: Not leader")
	}

	sorted := make([]int, len(rf.matchIndex))
	copy(sorted, rf.matchIndex)

	sort.Ints(sorted)
	var mid int
	if len(sorted)%2 == 0 {
		mid = len(sorted)/2 - 1
	} else {
		mid = len(sorted) / 2
	}

	majority := sorted[mid]
	// if majority has a smaller term or is replaced by a snapshot, so is any other candidate of N
	// defined as N = sorted[i], i < mid
	if majority > rf.commitIndex && rf.logs.termOf(majority) == rf.currentTerm {
		rf.commitIndex = majority
		rf.applyCommitted()
	}
}

// Thread safety: Unsafe
func (rf *Raft) applyCommitted() {
	for rf.commitIndex > rf.lastApplied {
		index := rf.lastApplied + 1

		if rf.logs.isLive(index) {
			entry := rf.logs.get(index)
			msg := ApplyMsg{
				CommandValid: true,
				Command:      entry.Command,
				CommandIndex: index,
			}
			logger.PrintfLn(LogInfo, "[%v%v] Apply command %v", rf.me, rf.role, index)
			rf.applyCh <- msg
		}

		rf.lastApplied = index
	}
}

// Thread safety: Unsafe
func (rf *Raft) cancelTimeout() *Context {
	rf.ctx.Cancel()
	ctx := NewCancellable()
	rf.ctx = ctx
	return ctx
}

// Thread safety: Unsafe
func (rf *Raft) setElectionTimeout() {
	ctx := rf.cancelTimeout()
	go rf.electionTimeout(ctx)
}

// Thread safety: Unsafe
func (rf *Raft) setHeartbeatTicker() {
	ctx := rf.cancelTimeout()
	go rf.heartbeatTicker(ctx)
}

// Thread safety: Unsafe
func (rf *Raft) revertToFollowerKeepVote() {
	logger.PrintfLn(LogInfo, "[%v%v] Role Shift: Reader", rf.me, rf.role)
	rf.role = Follower

	rf.nextIndex = nil
	rf.matchIndex = nil

	rf.setElectionTimeout()
}

// Thread safety: Unsafe
func (rf *Raft) revertToFollower() {
	rf.revertToFollowerKeepVote()
	rf.votedFor = NOBODY
}

// Thread safety: Unsafe
func (rf *Raft) appendRequestFrom(nextIndex int) AppendEntriesArgs {
	prevLogIndex := nextIndex - 1
	prevLogTerm := rf.logs.termOf(prevLogIndex)
	entries := rf.logs.entriesStartFrom(nextIndex)
	copied := make([]LogEntry, len(entries))
	copy(copied, entries)

	return AppendEntriesArgs{
		Term:         rf.currentTerm,
		LeaderId:     rf.me,
		PrevLogIndex: prevLogIndex,
		PrevLogTerm:  prevLogTerm,
		Entries:      copied,
		LeaderCommit: rf.commitIndex,
	}
}

// Thread safety: Unsafe
func (rf *Raft) startLeadership() {
	rf.role = Leader
	rf.nextIndex = make([]int, len(rf.peers))
	rf.matchIndex = make([]int, len(rf.peers))
	for i := 0; i < len(rf.nextIndex); i++ {
		rf.nextIndex[i] = rf.logs.lastIndex() + 1
		rf.matchIndex[i] = 0
	}
	rf.matchIndex[rf.me] = rf.logs.lastIndex()

	rf.setHeartbeatTicker()
}

// Thread safety: Unsafe
func (rf *Raft) startElection() {
	logger.PrintfLn(LogInfo, "[%v%v] Role Shift: Candidate", rf.me, rf.role)
	rf.currentTerm += 1
	rf.role = Candidate

	logger.PrintfLn(LogInfo, "[%v%v] Started election %v", rf.me, rf.role, rf.currentTerm)

	votes := make(map[int]bool)
	votes[rf.me] = true
	rf.votedFor = rf.me

	args := RequestVoteArgs{
		Term:         rf.currentTerm,
		CandidateId:  rf.me,
		LastLogIndex: rf.logs.lastIndex(),
		LastLogTerm:  rf.logs.lastTerm(),
	}

	for server := range rf.peers {
		if server != rf.me {
			logger.PrintfLn(LogDebug, "[%v%v] Send to [%v] %v", rf.me, rf.role, server, &args)
			go func(server int) {
				reply := RequestVoteReply{}
				if rf.sendRequestVote(server, &args, &reply) {
					rf.handleRequestVoteReply(votes, server, &args, &reply)
				}
			}(server)
		}
	}
}

// example RequestVote RPC arguments structure.
// field names must start with capital letters!
type RequestVoteArgs struct {
	// Your data here (2A, 2B).
	Term         int
	CandidateId  int
	LastLogIndex int
	LastLogTerm  int
}

func (args *RequestVoteArgs) String() string {
	return fmt.Sprintf("RequestVoteArgs { Term: %v, CandidateId: %v, LastLogIndex: %v, LastLogTerm: %v }", args.Term, args.CandidateId, args.LastLogIndex, args.LastLogTerm)
}

// example RequestVote RPC reply structure.
// field names must start with capital letters!
type RequestVoteReply struct {
	// Your data here (2A).
	Term        int
	VoteGranted bool
}

func (reply *RequestVoteReply) String() string {
	return fmt.Sprintf("RequestVoteReply { Term: %v, VoteGranded: %v }", reply.Term, reply.VoteGranted)
}

// Thread safety: Safe, Locking
func (rf *Raft) RequestVote(args *RequestVoteArgs, reply *RequestVoteReply) {
	// Your code here (2A, 2B).
	rf.mu.Lock()
	defer rf.mu.Unlock()
	defer logger.PrintfLn(LogDebug, "[%v%v] Reply %v", rf.me, rf.role, reply)

	logger.PrintfLn(LogDebug, "[%v%v] Recv %v", rf.me, rf.role, args)

	if rf.updateCurrentTerm(args.Term) {
		rf.revertToFollower()
	}
	reply.Term = rf.currentTerm

	if rf.currentTerm > args.Term {
		reply.VoteGranted = false
		return
	}

	if moreUpToDate(rf.logs.lastTerm(), rf.logs.lastIndex(), args.LastLogTerm, args.LastLogIndex) {
		reply.VoteGranted = false
		return
	}

	if rf.role == Follower && (rf.votedFor == NOBODY || rf.votedFor == args.CandidateId) {
		rf.votedFor = args.CandidateId
		reply.VoteGranted = true
		rf.setElectionTimeout()
	} else {
		reply.VoteGranted = false
	}

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
	Term         int
	LeaderId     int
	PrevLogIndex int
	PrevLogTerm  int
	Entries      []LogEntry
	LeaderCommit int
}

func (args *AppendEntriesArgs) String() string {
	return fmt.Sprintf("AppendEntries { Term: %v, LeaderId: %v, PrevLogIndex: %v, PrevLogTerm: %v, len(Entries): %v, LeaderCommit: %v }", args.Term, args.LeaderId, args.PrevLogIndex, args.PrevLogTerm, len(args.Entries), args.LeaderCommit)
}

func (args *AppendEntriesArgs) lastIndex() int {
	return args.PrevLogIndex + len(args.Entries)
}

type AppendEntriesReply struct {
	Term    int
	Success bool
}

func (reply *AppendEntriesReply) String() string {
	return fmt.Sprintf("AppendEntriesReply { Term: %v, Success: %v }", reply.Term, reply.Success)
}

// Thread safety: Safe, Locking
func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	defer logger.PrintfLn(LogDebug, "[%v%v] Reply %v", rf.me, rf.role, reply)

	logger.PrintfLn(LogDebug, "[%v%v] Recv %v", rf.me, rf.role, args)

	// the difference to currentTerm > args.Term is it's still the same term, votedFor should not be reset
	if rf.currentTerm == args.Term {
		rf.revertToFollowerKeepVote()
	}
	if rf.updateCurrentTerm(args.Term) {
		rf.revertToFollower()
	}

	reply.Term = rf.currentTerm
	if rf.currentTerm > args.Term {
		reply.Success = false
		return
	}

	if args.PrevLogIndex > rf.logs.lastIndex() {
		reply.Success = false
		return
	}
	prevTerm := rf.logs.termOf(args.PrevLogIndex)
	// snapshot is always committed, always correct
	if prevTerm >= 0 && prevTerm != args.PrevLogTerm {
		reply.Success = false
		return
	}

	rf.logs.update(args.PrevLogIndex, args.Entries)
	logger.PrintfLn(LogDebug, "[%v%v] Log %v", rf.me, rf.role, &rf.logs)

	if args.LeaderCommit > rf.commitIndex {
		rf.commitIndex = min(args.LeaderCommit, args.lastIndex())
		rf.applyCommitted()
	}

	reply.Success = true
}

// Thread safety: Depends on the source of args and reply
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
func (rf *Raft) Start(command interface{}) (index int, term int, isLeader bool) {
	// Your code here (2B).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.role != Leader {
		return -1, -1, false
	}

	index = rf.logs.append(command, rf.currentTerm)
	logger.PrintfLn(LogDebug, "[%v%v] Log %v", rf.me, rf.role, &rf.logs)
	logger.PrintfLn(LogInfo, "[%v%v] Started new command at term %v, index %v", rf.me, rf.role, rf.currentTerm, index)

	rf.matchIndex[rf.me] = index
	rf.nextIndex[rf.me] = index + 1

	for server := range rf.peers {
		if server == rf.me {
			continue
		}

		go rf.appendEntriesLoop(server)
	}

	return index, rf.currentTerm, true
}

func (rf *Raft) appendEntriesRound(server int) bool {
	rf.mu.Lock()
	if rf.role != Leader {
		rf.mu.Unlock()
		return false
	}

	nextIndex := rf.nextIndex[server]
	if rf.logs.lastIndex() < nextIndex {
		rf.mu.Unlock()
		return false
	}

	if !rf.logs.isLive(nextIndex) {
		panic("snapshot not implemented")
	}
	args := rf.appendRequestFrom(nextIndex)
	reply := AppendEntriesReply{}

	rf.mu.Unlock()

	logger.PrintfLn(LogDebug, "[%v%v] Send to [%v] %v", rf.me, rf.role, server, &args)
	if !rf.sendAppendEntry(server, &args, &reply) {
		return true
	}

	return rf.handleAppendEntriesReply(server, &args, &reply)
}

func (rf *Raft) handleAppendEntriesReply(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	rf.mu.Lock()
	logger.PrintfLn(LogDebug, "[%v%v] Recv from [%v] %v", rf.me, rf.role, server, &reply)

	if rf.updateCurrentTerm(reply.Term) {
		rf.revertToFollower()
		rf.mu.Unlock()
		return false
	}

	if rf.currentTerm > args.Term {
		rf.mu.Unlock()
		return false
	}

	if reply.Success {
		rf.matchIndex[server] = max(rf.matchIndex[server], args.lastIndex())
		rf.nextIndex[server] = max(rf.nextIndex[server], args.lastIndex()+1)

		rf.leaderUpdateCommitted()
		rf.mu.Unlock()
		return false
	} else {
		rf.nextIndex[server] -= 1
		rf.mu.Unlock()
		return true
	}
}

// Thread safety: Safe, No lock
func (rf *Raft) appendEntriesLoop(server int) {
	for !rf.killed() {
		retry := rf.appendEntriesRound(server)
		if !retry {
			return
		}
	}
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
}

// Thread safety: Safe, No lock
func (rf *Raft) killed() bool {
	z := atomic.LoadInt32(&rf.dead)
	return z == 1
}

// Thread safety: Unsafe
func (rf *Raft) heartbeat() {
	for server := range rf.peers {
		if server == rf.me {
			continue
		}

		args := rf.appendRequestFrom(rf.nextIndex[server])
		logger.PrintfLn(LogDebug, "[%v%v] Send heartbeat to [%v]", rf.me, rf.role, server)
		go func(server int) {
			reply := AppendEntriesReply{}
			if rf.sendAppendEntry(server, &args, &reply) {
				rf.handleAppendEntriesReply(server, &args, &reply)
			}
		}(server)
	}
}

// Thread safety: Safe, Locking
func (rf *Raft) handleRequestVoteReply(
	votes map[int]bool,
	server int,
	args *RequestVoteArgs,
	reply *RequestVoteReply,
) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.updateCurrentTerm(reply.Term) {
		rf.revertToFollower()
		return
	}

	// cancelled by role shift, candidate timeout, or vote from a previous election
	if rf.currentTerm > args.Term || rf.role != Candidate {
		return
	}

	votes[server] = reply.VoteGranted
	total := len(rf.peers)
	granted := 0
	for _, v := range votes {
		if v {
			granted += 1
		}
	}

	if granted > total/2 {
		// won the election
		// cancel everything, start leadership
		logger.PrintfLn(LogDebug,
			"[%v%v] Role Shift: Leader %v (%v/%v votes)", rf.me, rf.role, rf.currentTerm, granted, total)
		rf.startLeadership()
	}
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
	rf.logs = MakeLogs()
	rf.currentTerm = 0
	rf.votedFor = NOBODY

	rf.role = Follower
	rf.applyCh = applyCh
	rf.commitIndex = 0
	rf.lastApplied = 0
	rf.ctx = NewCancellable()

	rf.nextIndex = nil
	rf.matchIndex = nil

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())

	// start ticker goroutine to start elections
	go rf.electionTimeout(rf.ctx)

	return rf
}
