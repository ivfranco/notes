package mr

//
// RPC definitions.
//
// remember to capitalize all names.
//

import (
	"os"
	"strconv"
)

//
// example to show how to declare the arguments
// and reply for an RPC.
//

type ExampleArgs struct {
	X int
}

type ExampleReply struct {
	Y int
}

type GetWorkArgs struct{}

type WorkType int

const (
	Map WorkType = iota
	Reduce
	Saturated
	None
)

type MapWorkload struct {
	File    string
	Id      int
	NReduce int
}

type ReduceWorkload struct {
	Hash int
}

type GetWorkReply struct {
	Type WorkType
	// only valid when Type == Map
	MapWorkload MapWorkload
	// only valid when Type == Reduce
	ReduceWorkload ReduceWorkload
}

type WorkDoneArgs struct {
	Type WorkType
	Id   int
}

type WorkDoneReply struct{}

// Add your RPC definitions here.

// Cook up a unique-ish UNIX-domain socket name
// in /var/tmp, for the coordinator.
// Can't use the current directory since
// Athena AFS doesn't support UNIX-domain sockets.
func coordinatorSock() string {
	s := "/var/tmp/824-mr-"
	s += strconv.Itoa(os.Getuid())
	return s
}
