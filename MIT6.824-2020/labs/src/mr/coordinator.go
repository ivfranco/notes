package mr

import (
	"log"
	"net"
	"net/http"
	"net/rpc"
	"os"
	"sync"
	"time"
)

const (
	JOB_TIMEOUT time.Duration = time.Second * 10
)

type JobState int

const (
	Idle JobState = iota
	Issued
	Done
)

type MapJob struct {
	file  string
	state JobState
	// only valid when state == Issued
	due time.Time
}

type ReduceJob struct {
	state JobState
	// only valid when state == Issued
	due time.Time
}

type Coordinator struct {
	mu         sync.Mutex
	nReduce    int
	mapJobs    []MapJob
	reduceJobs []ReduceJob
}

// Your code here -- RPC handlers for the worker to call.

// an example RPC handler.
//
// the RPC argument and reply types are defined in rpc.go.
func (c *Coordinator) Example(args *ExampleArgs, reply *ExampleReply) error {
	reply.Y = args.X + 1
	return nil
}

func (c *Coordinator) GetWork(args *GetWorkArgs, reply *GetWorkReply) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	done := true

	// issue map works
	for i := 0; i < len(c.mapJobs); i++ {
		job := &c.mapJobs[i]

		if job.state == Issued && job.due.Before(time.Now()) {
			job.state = Idle
		}

		switch job.state {
		case Idle:
			*reply = GetWorkReply{
				Type: Map,
				MapWorkload: MapWorkload{
					File:    job.file,
					Id:      i,
					NReduce: c.nReduce,
				},
			}

			job.state = Issued
			job.due = time.Now().Add(JOB_TIMEOUT)

			return nil
		case Issued:
			done = false
		}
	}

	// wait until all map jobs are completed
	if !done {
		reply.Type = Saturated
		return nil
	}

	// issue reduce works
	for i := 0; i < len(c.reduceJobs); i++ {
		job := &c.reduceJobs[i]

		if job.state == Issued && job.due.Before(time.Now()) {
			job.state = Idle
		}

		switch job.state {
		case Idle:
			*reply = GetWorkReply{
				Type: Reduce,
				ReduceWorkload: ReduceWorkload{
					Hash: i,
				},
			}

			job.state = Issued
			job.due = time.Now().Add(JOB_TIMEOUT)

			return nil
		case Issued:
			done = false
		}
	}

	if done {
		reply.Type = None
	} else {
		reply.Type = Saturated
	}

	return nil
}

func (c *Coordinator) WorkDone(args *WorkDoneArgs, reply *WorkDoneReply) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	switch args.Type {
	case Map:
		c.mapJobs[args.Id].state = Done
	case Reduce:
		c.reduceJobs[args.Id].state = Done
	default:
		log.Fatalln("Coordinator.WorkDone: Unexpected enum value")
	}

	return nil
}

// start a thread that listens for RPCs from worker.go
func (c *Coordinator) server() {
	rpc.Register(c)
	rpc.HandleHTTP()
	//l, e := net.Listen("tcp", ":1234")
	sockname := coordinatorSock()
	os.Remove(sockname)
	l, e := net.Listen("unix", sockname)
	if e != nil {
		log.Fatal("listen error:", e)
	}
	go http.Serve(l, nil)
}

// main/mrcoordinator.go calls Done() periodically to find out
// if the entire job has finished.
func (c *Coordinator) Done() bool {
	c.mu.Lock()
	defer c.mu.Unlock()

	ret := true

	// Your code here.
	for _, job := range c.mapJobs {
		ret = ret && job.state == Done
	}
	for _, job := range c.reduceJobs {
		ret = ret && job.state == Done
	}

	return ret
}

// create a Coordinator.
// main/mrcoordinator.go calls this function.
// nReduce is the number of reduce tasks to use.
func MakeCoordinator(files []string, nReduce int) *Coordinator {
	mapJobs := []MapJob{}
	for _, f := range files {
		mapJobs = append(mapJobs, MapJob{
			file:  f,
			state: Idle,
		})
	}

	reduceJobs := []ReduceJob{}
	for i := 0; i < nReduce; i++ {
		reduceJobs = append(reduceJobs, ReduceJob{
			state: Idle,
		})
	}

	c := Coordinator{
		nReduce:    nReduce,
		mapJobs:    mapJobs,
		reduceJobs: reduceJobs,
	}

	c.server()
	return &c
}
