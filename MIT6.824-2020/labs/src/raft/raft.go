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
	"bytes"
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

	"6.824/labgob"
	"6.824/labrpc"
)

const (
	SLOW_MOTION          time.Duration = time.Millisecond * 3
	HEARTBEAT_INTERVAL   time.Duration = SLOW_MOTION * 50
	MIN_ELECTION_TIMEOUT time.Duration = SLOW_MOTION * 150
	MAX_ELECTION_TIMEOUT time.Duration = SLOW_MOTION * 300

	// Valid server id starts from 0
	NOBODY int = -1
	// Invalid index and term, used sparsely in example code
	NONE int = -1
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
	Snapshot          []byte
	LastIncludedIndex int
	LastIncludedTerm  int
	LiveLogs          []LogEntry
}

func (l *Logs) String() string {
	terms := make([]string, 0, len(l.LiveLogs))
	for i, entry := range l.LiveLogs {
		terms = append(terms, strconv.Itoa(l.unTranslateIndex(i))+":"+strconv.Itoa(entry.Term))
	}
	ss := fmt.Sprintf("(%v:%v)", l.LastIncludedIndex, l.LastIncludedTerm)
	return "Logs " + ss + "(" + strings.Join(terms, "|") + ")"
}

func MakeLogs() Logs {
	return Logs{
		Snapshot:          nil,
		LastIncludedIndex: 0,
		LastIncludedTerm:  0,
		LiveLogs:          make([]LogEntry, 0),
	}
}

func (l *Logs) lastIndex() int {
	return l.LastIncludedIndex + len(l.LiveLogs)
}

func (l *Logs) lastTerm() int {
	return l.termOf(l.lastIndex())
}

func (l *Logs) append(command interface{}, term int) int {
	l.LiveLogs = append(l.LiveLogs, LogEntry{Command: command, Term: term})
	return l.lastIndex()
}

func (l *Logs) isLive(nextIndex int) bool {
	return nextIndex > l.LastIncludedIndex
}

func (l *Logs) translateIndex(index int) int {
	return index - l.LastIncludedIndex - 1
}

func (l *Logs) unTranslateIndex(index int) int {
	return index + l.LastIncludedIndex + 1
}

func (l *Logs) get(index int) *LogEntry {
	return &l.LiveLogs[l.translateIndex(index)]
}

func (l *Logs) termOf(index int) int {
	if index > l.lastIndex() {
		// querying term of non-existing log entry
		return NONE
	} else if l.isLive(index) {
		return l.get(index).Term
	} else if index == l.LastIncludedIndex {
		return l.LastIncludedTerm
	} else {
		// querying term of log entry replaced by snapshot
		return NONE
	}
}

func (l *Logs) entriesStartFrom(index int) []LogEntry {
	return l.LiveLogs[l.translateIndex(index):]
}

func (l *Logs) detectConflict(prevIndex int, prevTerm int, entries []LogEntry) (int, int) {
	xTerm := NONE
	xIndex := NONE

	if term := l.termOf(prevIndex); term != NONE && term != prevTerm {
		xTerm = term
		xIndex = l.translateIndex(prevIndex)
	} else {
		for i, entry := range entries {
			j := prevIndex + i + 1
			// skip snapshot
			if !l.isLive(j) {
				continue
			}

			k := l.translateIndex(j)
			if k >= len(l.LiveLogs) {
				break
			} else if l.LiveLogs[k].Term != entry.Term {
				xTerm = l.LiveLogs[k].Term
				xIndex = k
				break
			}
		}
	}

	// backtrack to the first index with the same term, xIndex > 0 also handles NONE
	for xIndex > 0 && l.LiveLogs[xIndex-1].Term == xTerm {
		xIndex -= 1
	}
	if xIndex != NONE {
		xIndex = l.unTranslateIndex(xIndex)
	}

	return xTerm, xIndex
}

func (l *Logs) update(prevIndex int, entries []LogEntry) bool {
	shouldPersist := false

	for i, entry := range entries {
		j := prevIndex + i + 1
		// skip snapshot
		if !l.isLive(j) {
			continue
		}

		k := l.translateIndex(j)
		if k+1 > len(l.LiveLogs) {
			shouldPersist = true
			l.LiveLogs = append(l.LiveLogs, entry)
		} else if l.LiveLogs[k].Term != entry.Term {
			// conflicting log entries, happens exactly once as all following entries are deleted
			shouldPersist = true
			l.LiveLogs = l.LiveLogs[:k]
			l.LiveLogs = append(l.LiveLogs, entry)
		}
	}

	return shouldPersist
}

func (l *Logs) nextIndexFromReply(xTerm int, xIndex int, xLen int) int {
	// case 3: XTerm == XIndex == NONE
	if xTerm == NONE && xIndex == NONE {
		return xLen
	}

	// case 2: leader has entries in XTerm
	for i := len(l.LiveLogs) - 1; i >= 0; i -= 1 {
		if l.LiveLogs[i].Term == xTerm {
			return l.unTranslateIndex(i) + 1
		}
	}

	if l.LastIncludedTerm == xTerm {
		return l.LastIncludedIndex + 1
	}

	// case 1: leader has no entry in XTerm
	return xIndex
}

func (l *Logs) moreUpToDate(term int, index int) bool {
	return moreUpToDate(l.lastTerm(), l.lastIndex(), term, index)
}

func (l *Logs) shouldInstallSnapshot(lastIncludedIndex int, lastIncludedTerm int) bool {
	return moreUpToDate(lastIncludedTerm, lastIncludedIndex, l.LastIncludedTerm, l.LastIncludedIndex)
}

func (l *Logs) installSnapshot(snapshot []byte, lastIncludedIndex int, lastIncludedTerm int) bool {
	if !l.shouldInstallSnapshot(lastIncludedIndex, lastIncludedTerm) {
		panic("Should check staleness first")
	}

	var shouldReset bool

	skip := 0
	for skip < len(l.LiveLogs) && l.unTranslateIndex(skip) < lastIncludedIndex {
		l.LiveLogs[skip] = LogEntry{}
		skip += 1
	}

	if skip < len(l.LiveLogs) &&
		l.unTranslateIndex(skip) == lastIncludedIndex &&
		l.LiveLogs[skip].Term == lastIncludedTerm {
		// no conflict, retain existing logs after skip
		l.LiveLogs[skip] = LogEntry{}
		l.LiveLogs = l.LiveLogs[skip+1:]
		shouldReset = false
	} else {
		// drop entire log, should reset state machine
		l.LiveLogs = make([]LogEntry, 0)
		shouldReset = true
	}

	l.Snapshot = snapshot
	l.LastIncludedIndex = lastIncludedIndex
	l.LastIncludedTerm = lastIncludedTerm

	return shouldReset
}

type ShouldPersist struct {
	LastIncludedIndex int
	LastIncludedTerm  int
	LiveLogs          []LogEntry
	CurrentTerm       int
	VotedFor          int
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

	// const fields, initialized once on startup, should not be reassigned
	applyCh   chan ApplyMsg // Channel for commands and snapshots
	applyCond *sync.Cond    // Conditional variable guarding the command applier

	// Non-volatile, persist on modification
	logs        Logs // Logs and snapshot of the Raft server
	currentTerm int  // Highest term perceived by this Raft server
	votedFor    int  // To which peer this server granted vote during this election

	// Volatile
	role        Role     // One of Leader, Candidate and Follower
	commitIndex int      // Index of highest log entry known to be committed
	lastApplied int      // Index of highest log entry applied to state machine
	ctx         *Context // Cancellable context controlling the current timeout

	// Volatile, Leader only
	nextIndex  []int // For each server, index of the next log entry to send to that server
	matchIndex []int // For each server, index of highest log entry known to be replicated on server
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

// Thread safety: Unsafe
//
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

	w := &bytes.Buffer{}
	enc := labgob.NewEncoder(w)

	state := ShouldPersist{
		LastIncludedIndex: rf.logs.LastIncludedIndex,
		LastIncludedTerm:  rf.logs.LastIncludedTerm,
		LiveLogs:          rf.logs.LiveLogs,
		CurrentTerm:       rf.currentTerm,
		VotedFor:          rf.votedFor,
	}
	if err := enc.Encode(state); err != nil {
		logger.FatalfLn("Encoding error: %v", err)
	}

	rf.persister.SaveStateAndSnapshot(w.Bytes(), rf.logs.Snapshot)
}

// Thread safety: Unsafe
//
// restore previously persisted state.
func (rf *Raft) readPersist(data []byte, snapshot []byte) {
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

	r := bytes.NewBuffer(data)
	dec := labgob.NewDecoder(r)
	state := ShouldPersist{}
	if err := dec.Decode(&state); err != nil {
		logger.FatalfLn("Decode error: %v", err)
	}

	rf.logs.Snapshot = snapshot
	rf.logs.LastIncludedIndex = state.LastIncludedIndex
	rf.logs.LastIncludedTerm = state.LastIncludedTerm
	rf.logs.LiveLogs = state.LiveLogs
	rf.currentTerm = state.CurrentTerm
	rf.votedFor = state.VotedFor
}

// A service wants to switch to snapshot.  Only do so if Raft hasn't
// have more recent info since it communicate the snapshot on applyCh.
func (rf *Raft) CondInstallSnapshot(lastIncludedTerm int, lastIncludedIndex int, snapshot []byte) bool {
	// Your code here (2D).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	// same check as in Raft.InstallSnapshot
	if rf.commitIndex >= lastIncludedIndex {
		return false
	}

	rf.logs.installSnapshot(snapshot, lastIncludedIndex, lastIncludedTerm)
	rf.lastApplied = lastIncludedIndex
	rf.commitIndex = lastIncludedIndex
	rf.persist()

	return true
}

// the service says it has created a snapshot that has
// all info up to and including index. this means the
// service no longer needs the log through (and including)
// that index. Raft should now trim its log as much as possible.
func (rf *Raft) Snapshot(index int, snapshot []byte) {
	// Your code here (2D).
	rf.mu.Lock()
	defer rf.mu.Unlock()

	if rf.logs.LastIncludedIndex >= index {
		logger.PrintfLn(LogWarn, "[%v%v] Client provided snapshot older than local at %v", rf.me, rf.role, index)
	} else {
		logger.PrintfLn(LogInfo, "[%v%v] Install snapshot from client at %v", rf.me, rf.role, index)
		rf.logs.installSnapshot(snapshot, index, rf.logs.termOf(index))
		rf.persist()
	}

	// snapshot provided by a local server is always behind rf.lastApplied, rf.lastApplied and
	// rf.commitIndex are not affected
}

// Thread safety: Unsafe
func (rf *Raft) updateCurrentTerm(term int) (reset bool) {
	if rf.currentTerm < term {
		rf.currentTerm = term
		return true
	} else {
		return false
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
	if rf.role != Follower {
		logger.PrintfLn(LogInfo, "[%v%v] Role Shift: Follower", rf.me, rf.role)
	}

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

// Thread safety: Safe, Locking
func (rf *Raft) electionTimeout(ctx *Context) {
	time.Sleep(randomElectionTimeout())
	rf.mu.Lock()
	if !ctx.IsCancelled() && rf.role != Leader {
		rf.startElection()
		go rf.electionTimeout(rf.ctx)
	}
	rf.mu.Unlock()
}

// Thread safety: Safe, Locking
func (rf *Raft) heartbeatTicker(ctx *Context) {
	for !rf.killed() {
		rf.mu.Lock()
		if ctx.IsCancelled() || rf.role != Leader {
			rf.mu.Unlock()
			return
		}
		rf.heartbeat()
		rf.mu.Unlock()
		time.Sleep(HEARTBEAT_INTERVAL)
	}
}

// Thread safety: Unsafe
func (rf *Raft) heartbeat() {
	for server := range rf.peers {
		if server == rf.me {
			continue
		}

		logger.PrintfLn(LogDebug, "[%v%v] Send heartbeat to [%v]", rf.me, rf.role, server)
		from := rf.logs.lastIndex() + 1
		args := rf.appendEntriesArgsFrom(from)
		go func(server int) {
			reply := AppendEntriesReply{}
			if rf.sendAppendEntry(server, &args, &reply) {
				rf.handleAppendEntriesReply(server, &args, &reply)
			}
		}(server)
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
	if rf.shouldCommit() {
		rf.applyCond.Broadcast()
	}
}

// Thread safety: Unsafe
func (rf *Raft) shouldCommit() bool {
	return rf.lastApplied < rf.commitIndex
}

// Thread safety: Safe, Locking
func (rf *Raft) applier() {
	for !rf.killed() {
		rf.mu.Lock()

		for !rf.shouldCommit() {
			rf.applyCond.Wait()
		}

		index := rf.lastApplied + 1
		// the only results in
		if !rf.logs.isLive(index) {
			logger.FatalfLn("[%v%v] lastApplied+1 points to snapshot", rf.me, rf.role)
		}
		entries := make([]LogEntry, 0)
		for i := 0; index+i <= rf.commitIndex; i++ {
			entries = append(entries, *rf.logs.get(index + i))
		}

		rf.mu.Unlock()

		for i, entry := range entries {
			msg := ApplyMsg{
				CommandValid: true,
				Command:      entry.Command,
				CommandIndex: index + i,
			}
			rf.applyCh <- msg
		}

		rf.mu.Lock()
		lastAppliedIndex := index + len(entries) - 1
		logger.PrintfLn(LogInfo, "[%v%v] Applied commands %v to %v", rf.me, rf.role, index, lastAppliedIndex)
		logger.PrintfLn(LogInfo, "[%v%v] %v", rf.me, rf.role, &rf.logs)
		// a snapshot may have been installed between Unlock and Lock
		rf.lastApplied = max(lastAppliedIndex, rf.lastApplied)
		rf.mu.Unlock()
	}
}

// Thread safety: Unsafe
func (rf *Raft) appendEntriesArgsFrom(nextIndex int) AppendEntriesArgs {
	nextIndex = min(nextIndex, rf.logs.lastIndex()+1)
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
func (rf *Raft) installSnapshotArgs() InstallSnapshotArgs {
	return InstallSnapshotArgs{
		Term:              rf.currentTerm,
		LeaderId:          rf.me,
		LastIncludedIndex: rf.logs.LastIncludedIndex,
		LastIncludedTerm:  rf.logs.LastIncludedTerm,
		Data:              rf.logs.Snapshot,
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
	rf.updateCurrentTerm(rf.currentTerm + 1)
	if rf.role != Candidate {
		logger.PrintfLn(LogInfo, "[%v%v] Role Shift: Candidate", rf.me, rf.role)
	}
	rf.role = Candidate

	logger.PrintfLn(LogInfo, "[%v%v] Started election %v", rf.me, rf.role, rf.currentTerm)

	votes := make(map[int]bool)
	votes[rf.me] = true
	rf.votedFor = rf.me

	rf.persist()

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
	Term         int // candidate’s term
	CandidateId  int // candidate requesting vote
	LastLogIndex int // index of candidate’s last log entry
	LastLogTerm  int // term of candidate’s last log entry

}

func (args *RequestVoteArgs) String() string {
	return fmt.Sprintf("RequestVoteArgs { Term: %v, CandidateId: %v, LastLogIndex: %v, LastLogTerm: %v }", args.Term, args.CandidateId, args.LastLogIndex, args.LastLogTerm)
}

// example RequestVote RPC reply structure.
// field names must start with capital letters!
type RequestVoteReply struct {
	// Your data here (2A).
	Term        int  // currentTerm, for candidate to update itself
	VoteGranted bool // true means candidate received vote
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

	shouldPersist := false
	defer func() {
		if shouldPersist {
			rf.persist()
		}
	}()

	logger.PrintfLn(LogDebug, "[%v%v] Recv %v", rf.me, rf.role, args)

	if rf.updateCurrentTerm(args.Term) {
		shouldPersist = true
		rf.revertToFollower()
	}
	reply.Term = rf.currentTerm

	if rf.currentTerm > args.Term {
		reply.VoteGranted = false
		return
	}

	if rf.logs.moreUpToDate(args.LastLogTerm, args.LastLogIndex) {
		reply.VoteGranted = false
		return
	}

	if rf.role == Follower && (rf.votedFor == NOBODY || rf.votedFor == args.CandidateId) {
		rf.votedFor = args.CandidateId
		shouldPersist = true
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

// Thread safety: Safe, Locking
func (rf *Raft) handleRequestVoteReply(
	votes map[int]bool,
	server int,
	args *RequestVoteArgs,
	reply *RequestVoteReply,
) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	shouldPersist := false
	defer func() {
		if shouldPersist {
			rf.persist()
		}
	}()

	logger.PrintfLn(LogDebug, "[%v%v] Recv from %v %v", rf.me, rf.role, server, reply)

	if rf.updateCurrentTerm(reply.Term) {
		shouldPersist = true
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

type AppendEntriesArgs struct {
	Term         int        // leader’s term
	LeaderId     int        // so follower can redirect clients, un-used in lab 2
	PrevLogIndex int        // index of log entry immediately preceding new ones
	PrevLogTerm  int        // term of prevLogIndex entry
	Entries      []LogEntry // log entries to store
	LeaderCommit int        // leader’s commitIndex
}

func (args *AppendEntriesArgs) String() string {
	return fmt.Sprintf("AppendEntries { Term: %v, LeaderId: %v, PrevLogIndex: %v, PrevLogTerm: %v, len(Entries): %v, LeaderCommit: %v }", args.Term, args.LeaderId, args.PrevLogIndex, args.PrevLogTerm, len(args.Entries), args.LeaderCommit)
}

func (args *AppendEntriesArgs) lastIndex() int {
	return args.PrevLogIndex + len(args.Entries)
}

type AppendEntriesReply struct {
	Term    int  // currentTerm, for leader to update itself
	Success bool // true if follower contained entry matching
	XTerm   int  // first term containing conflicting entries
	XIndex  int  // index to first entry in XTerm
	XLen    int  // length of logs, 1 + last index
}

func (reply *AppendEntriesReply) String() string {
	return fmt.Sprintf("AppendEntriesReply { Term: %v, Success: %v, XTerm: %v, XIndex: %v, XLen: %v}", reply.Term, reply.Success, reply.XTerm, reply.XIndex, reply.XLen)
}

// Thread safety: Safe, Locking
func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	logger.PrintfLn(LogDebug, "[%v%v] Recv %v", rf.me, rf.role, args)
	defer logger.PrintfLn(LogDebug, "[%v%v] Reply %v", rf.me, rf.role, reply)
	logger.PrintfLn(LogDebug, "[%v%v] %v", rf.me, rf.role, &rf.logs)

	shouldPersist := false
	defer func() {
		if shouldPersist {
			rf.persist()
		}
	}()

	// the difference to currentTerm > args.Term is it's still the same term, votedFor should not be reset
	if rf.currentTerm == args.Term {
		rf.revertToFollowerKeepVote()
	}
	if rf.updateCurrentTerm(args.Term) {
		shouldPersist = true
		rf.revertToFollower()
	}

	reply.Term = rf.currentTerm
	if rf.currentTerm > args.Term {
		reply.Success = false
		return
	}

	reply.XTerm, reply.XIndex = rf.logs.detectConflict(args.PrevLogIndex, args.PrevLogTerm, args.Entries)
	reply.XLen = rf.logs.lastIndex() + 1

	if args.PrevLogIndex > rf.logs.lastIndex() {
		reply.Success = false
		return
	}
	prevTerm := rf.logs.termOf(args.PrevLogIndex)
	// prevTerm == -1 if index is replaced by snapshot, snapshot is always committed, always correct
	if prevTerm >= 0 && prevTerm != args.PrevLogTerm {
		reply.Success = false
		return
	}

	// order significant, otherwise Logs.update may be skipped because of shouldPersist == true
	shouldPersist = rf.logs.update(args.PrevLogIndex, args.Entries) || shouldPersist

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

// Thread safety: Safe, Locking
func (rf *Raft) appendEntriesRound(server int) bool {
	rf.mu.Lock()

	// AppendEntries issued in a previous term, terminate and don't retry
	if rf.role != Leader {
		rf.mu.Unlock()
		return false
	}

	nextIndex := rf.nextIndex[server]
	// Follower is up-to-date, terminate and don't retry
	if rf.logs.lastIndex() < nextIndex {
		rf.mu.Unlock()
		return false
	}

	if !rf.logs.isLive(nextIndex) {
		args := rf.installSnapshotArgs()
		reply := InstallSnapshotReply{}

		logger.PrintfLn(LogDebug, "[%v%v] Send to %v %v", rf.me, rf.role, server, &args)
		rf.mu.Unlock()

		if rf.sendInstallSnapshot(server, &args, &reply) {
			rf.handleInstallSnapshotReply(&args, &reply)
		}

		// even if InstallSnapshot RPC succeeded, Leader still has to update nextIndex and
		// matchIndex by AppendEntries RPC
		return true
	}

	args := rf.appendEntriesArgsFrom(nextIndex)
	reply := AppendEntriesReply{}

	logger.PrintfLn(LogDebug, "[%v%v] Send to [%v] %v", rf.me, rf.role, server, &args)
	rf.mu.Unlock()

	// RPC failed on the way forward or back, retry operation
	if !rf.sendAppendEntry(server, &args, &reply) {
		return true
	}

	// retry only when AppendEntries doesn't succeed
	return rf.handleAppendEntriesReply(server, &args, &reply)
}

func (rf *Raft) handleAppendEntriesReply(server int, args *AppendEntriesArgs, reply *AppendEntriesReply) bool {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	logger.PrintfLn(LogDebug, "[%v%v] Recv from [%v] %v", rf.me, rf.role, server, reply)
	defer logger.PrintfLn(LogDebug, "[%v%v] nextIndex: %v, matchIndex: %v", rf.me, rf.role, rf.nextIndex, rf.matchIndex)

	shouldPersist := false
	defer func() {
		if shouldPersist {
			rf.persist()
		}
	}()

	if rf.updateCurrentTerm(reply.Term) {
		shouldPersist = true
		rf.revertToFollower()
		return false
	}

	if rf.currentTerm > args.Term || rf.role != Leader {
		return false
	}

	if reply.Success {
		rf.matchIndex[server] = max(rf.matchIndex[server], args.lastIndex())
		rf.nextIndex[server] = max(rf.nextIndex[server], args.lastIndex()+1)

		rf.leaderUpdateCommitted()
		return false
	} else {
		rf.nextIndex[server] = rf.logs.nextIndexFromReply(reply.XTerm, reply.XIndex, reply.XLen)
		return true
	}
}

// Thread safety: Safe, No lock
func (rf *Raft) appendEntriesLoop(server int) {
	for !rf.killed() {
		if retry := rf.appendEntriesRound(server); !retry {
			return
		}
	}
}

type InstallSnapshotArgs struct {
	Term              int    // leader’s term
	LeaderId          int    // so follower can redirect clients
	LastIncludedIndex int    // the snapshot replaces all entries up through and including this index
	LastIncludedTerm  int    // term of lastIncludedIndex
	Data              []byte // raw bytes of the snapshot chunk
}

func (args *InstallSnapshotArgs) String() string {
	return fmt.Sprintf("InstallSnapshotArgs { Term: %v, LeaderId: %v, LastIncludedIndex: %v, LastIncludedTerm: %v, len(Data): %v }", args.Term, args.LeaderId, args.LastIncludedIndex, args.LastIncludedTerm, len(args.Data))
}

type InstallSnapshotReply struct {
	Term int // currentTerm, for leader to update itself
}

func (reply *InstallSnapshotReply) String() string {
	return fmt.Sprintf("InstallSnapshotReply { Term: %v }", reply.Term)
}

// Thread safety: Safe, Locking
func (rf *Raft) InstallSnapshot(args *InstallSnapshotArgs, reply *InstallSnapshotReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	logger.PrintfLn(LogDebug, "[%v%v] Recv %v", rf.me, rf.role, args)
	defer logger.PrintfLn(LogDebug, "[%v%v] Reply %v", rf.me, rf.role, reply)
	defer logger.PrintfLn(LogDebug, "[%v%v] %v", rf.me, rf.role, &rf.logs)

	shouldPersist := false
	defer func() {
		if shouldPersist {
			rf.persist()
		}
	}()

	// InstallSnapshot RPC should be treated similarly to an AppendEntries RPC, i.e. revert to
	// follower on current term and reset election timeout
	if rf.updateCurrentTerm(args.Term) {
		shouldPersist = true
		rf.revertToFollower()
	} else if rf.currentTerm == args.Term {
		rf.revertToFollowerKeepVote()
	}

	reply.Term = rf.currentTerm

	if rf.currentTerm > args.Term {
		return
	}

	// (Mentioned nowhere in the paper)
	// InstallSnapshot should only be triggered because nextIndex of a Follower has retreated to an
	// already replaced log entry, also reset the state machine to an old (already committed)
	// snapshot may cause rf.lastApplied to decrease, which must be monotonically increasing
	if rf.commitIndex >= args.LastIncludedIndex {
		return
	}

	rf.setElectionTimeout()

	go func() {
		msg := ApplyMsg{
			SnapshotValid: true,
			Snapshot:      args.Data,
			SnapshotTerm:  args.LastIncludedTerm,
			SnapshotIndex: args.LastIncludedIndex,
		}
		rf.applyCh <- msg
	}()
}

// Thread safety: Safe, No lock
func (rf *Raft) sendInstallSnapshot(server int, args *InstallSnapshotArgs, reply *InstallSnapshotReply) bool {
	return rf.peers[server].Call("Raft.InstallSnapshot", args, reply)
}

func (rf *Raft) handleInstallSnapshotReply(args *InstallSnapshotArgs, reply *InstallSnapshotReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()

	logger.PrintfLn(LogDebug, "[%v%v] Recv %v", rf.me, rf.role, reply)

	if rf.updateCurrentTerm(reply.Term) {
		rf.revertToFollower()
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
		return NONE, NONE, false
	}

	index = rf.logs.append(command, rf.currentTerm)
	logger.PrintfLn(LogDebug, "[%v%v] %v", rf.me, rf.role, &rf.logs)
	logger.PrintfLn(LogInfo, "[%v%v] Started new command at term %v, index %v", rf.me, rf.role, rf.currentTerm, index)

	rf.persist()

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
	rf.applyCh = applyCh
	rf.applyCond = sync.NewCond(&rf.mu)

	rf.logs = MakeLogs()
	rf.currentTerm = 0
	rf.votedFor = NOBODY

	rf.role = Follower
	rf.ctx = NewCancellable()

	rf.nextIndex = nil
	rf.matchIndex = nil

	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState(), persister.ReadSnapshot())
	// snapshot is always applied and committed, the service running Raft reads it from the
	// non-volatile storage before startup
	rf.commitIndex = rf.logs.LastIncludedIndex
	rf.lastApplied = rf.logs.LastIncludedIndex

	// start ticker goroutine to start elections
	go rf.electionTimeout(rf.ctx)
	go rf.applier()

	return rf
}
