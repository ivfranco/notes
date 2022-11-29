package mr

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"hash/fnv"
	"io/ioutil"
	"log"
	"net/rpc"
	"os"
	"regexp"
	"sort"
	"strconv"
	"time"
)

const (
	ERROR_TIMEOUT     time.Duration = time.Millisecond * 500
	SATURATED_TIMEOUT time.Duration = time.Millisecond * 1000
)

// Map functions return a slice of KeyValue.
type KeyValue struct {
	Key   string
	Value string
}

type MapF func(string, string) []KeyValue
type ReduceF func(string, []string) string

type ByKey []KeyValue

func (a ByKey) Len() int           { return len(a) }
func (a ByKey) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByKey) Less(i, j int) bool { return a[i].Key < a[j].Key }

type TempFile struct {
	file   *os.File
	writer *bufio.Writer
	name   string
}

func NewTempFile(path string) (*TempFile, error) {
	file, err := os.CreateTemp(path, "")
	if err != nil {
		return nil, err
	}

	writer := bufio.NewWriter(file)
	name := file.Name()

	temp := &TempFile{
		file:   file,
		writer: writer,
		name:   name,
	}

	return temp, nil
}

func (temp *TempFile) Close() error {
	// have to make sure the file is not closed twice, by Golang documentation closing twice is UB
	if temp.file == nil {
		return nil
	}

	err := temp.file.Close()
	if err != nil {
		return err
	}

	temp.file = nil
	temp.writer = nil
	return nil
}

func (temp *TempFile) Remove() error {
	return os.Remove(temp.name)
}

func (temp *TempFile) Rename(name string) error {
	err := os.Rename(temp.name, name)
	if err != nil {
		return err
	}

	temp.name = name
	return nil
}

func (temp TempFile) Flush() error {
	if temp.writer == nil {
		return errors.New("temp file closed")
	}

	return temp.writer.Flush()
}

func (temp TempFile) Write(p []byte) (int, error) {
	if temp.writer == nil {
		return 0, errors.New("temp file closed")
	}

	return temp.writer.Write(p)
}

// use ihash(key) % NReduce to choose the reduce
// task number for each KeyValue emitted by Map.
func ihash(key string) int {
	h := fnv.New32a()
	h.Write([]byte(key))
	return int(h.Sum32() & 0x7fffffff)
}

// main/mrworker.go calls this function.
func Worker(mapf MapF, reducef ReduceF) {
	for {
		reply := GetWorkReply{}
		if !call("Coordinator.GetWork", &GetWorkArgs{}, &reply) {
			time.Sleep(ERROR_TIMEOUT)
			continue
		}

		switch reply.Type {
		case Map:
			err := runMap(mapf, reply.MapWorkload)
			if err != nil {
				log.Println(err)
				continue
			}

			args := WorkDoneArgs{
				Type: Map,
				Id:   reply.MapWorkload.Id,
			}
			if !call("Coordinator.WorkDone", &args, &WorkDoneReply{}) {
				time.Sleep(ERROR_TIMEOUT)
			}
		case Reduce:
			err := runReduce(reducef, reply.ReduceWorkload)
			if err != nil {
				log.Println(err)
				continue
			}

			args := WorkDoneArgs{
				Type: Reduce,
				Id:   reply.ReduceWorkload.Hash,
			}
			if !call("Coordinator.WorkDone", &args, &WorkDoneReply{}) {
				time.Sleep(ERROR_TIMEOUT)
			}
		case Saturated:
			time.Sleep(SATURATED_TIMEOUT)
		case None:
			return
		}
	}
}

func runMap(mapf MapF, workload MapWorkload) error {
	content, err := ioutil.ReadFile(workload.File)
	if err != nil {
		return err
	}

	temps := []*TempFile{}
	failed := true

	defer func() {
		if failed {
			for _, temp := range temps {
				temp.Close()
				temp.Remove()
			}
		}
	}()

	for i := 0; i < workload.NReduce; i++ {
		temp, err := NewTempFile("")
		if err != nil {
			return err
		}
		temps = append(temps, temp)
	}

	encoders := []*json.Encoder{}
	for _, temp := range temps {
		encoders = append(encoders, json.NewEncoder(temp))
	}

	kva := mapf(workload.File, string(content))
	for _, kv := range kva {
		hash := ihash(kv.Key) % workload.NReduce
		err := encoders[hash].Encode(kv)
		if err != nil {
			return err
		}
	}

	for hash, temp := range temps {
		err := temp.Flush()
		if err != nil {
			return err
		}

		err = temp.Close()
		if err != nil {
			return err
		}

		err = temp.Rename(fmt.Sprintf("mr-%v-%v", workload.Id, hash))
		if err != nil {
			return err
		}
	}

	failed = false
	return nil
}

func runReduce(reducef ReduceF, workload ReduceWorkload) error {
	pattern := regexp.MustCompile(`mr-(\d+)-(\d+)`)

	kva := []KeyValue{}

	entries, err := os.ReadDir(".")
	if err != nil {
		return err
	}

	for _, e := range entries {
		if !e.Type().IsRegular() || !pattern.MatchString(e.Name()) {
			continue
		}

		match := pattern.FindStringSubmatch(e.Name())
		hash, _ := strconv.Atoi(match[2])
		if hash != workload.Hash {
			continue
		}

		err = decodeKV(e.Name(), &kva)
		if err != nil {
			return err
		}
	}

	sort.Sort(ByKey(kva))

	ofile, err := NewTempFile("")
	if err != nil {
		return err
	}
	failed := true
	defer func() {
		if failed {
			ofile.Close()
			ofile.Remove()
		}
	}()

	i := 0
	for i < len(kva) {
		j := i + 1
		for j < len(kva) && kva[j].Key == kva[i].Key {
			j++
		}
		values := []string{}
		for k := i; k < j; k++ {
			values = append(values, kva[k].Value)
		}
		output := reducef(kva[i].Key, values)

		// this is the correct format for each line of Reduce output.
		_, err := fmt.Fprintf(ofile, "%v %v\n", kva[i].Key, output)
		if err != nil {
			return err
		}

		i = j
	}

	err = ofile.Flush()
	if err != nil {
		return err
	}
	err = ofile.Close()
	if err != nil {
		return err
	}
	err = ofile.Rename(fmt.Sprintf("mr-out-%v", workload.Hash))
	if err != nil {
		return err
	}

	failed = false
	return nil
}

func decodeKV(path string, kva *[]KeyValue) error {
	file, err := os.Open(path)
	if err != nil {
		return err
	}

	dec := json.NewDecoder(bufio.NewReader(file))
	for {
		var kv KeyValue
		err := dec.Decode(&kv)
		if err != nil {
			file.Close()
			break
		}
		*kva = append(*kva, kv)
	}

	return nil
}

// example function to show how to make an RPC call to the coordinator.
//
// the RPC argument and reply types are defined in rpc.go.
func CallExample() {

	// declare an argument structure.
	args := ExampleArgs{}

	// fill in the argument(s).
	args.X = 99

	// declare a reply structure.
	reply := ExampleReply{}

	// send the RPC request, wait for the reply.
	call("Coordinator.Example", &args, &reply)

	// reply.Y should be 100.
	fmt.Printf("reply.Y %v\n", reply.Y)
}

// send an RPC request to the coordinator, wait for the response.
// usually returns true.
// returns false if something goes wrong.
func call(rpcname string, args interface{}, reply interface{}) bool {
	// c, err := rpc.DialHTTP("tcp", "127.0.0.1"+":1234")
	sockname := coordinatorSock()
	c, err := rpc.DialHTTP("unix", sockname)
	if err != nil {
		log.Fatal("dialing:", err)
	}
	defer c.Close()

	err = c.Call(rpcname, args, reply)
	if err == nil {
		return true
	}

	fmt.Println(err)
	return false
}
