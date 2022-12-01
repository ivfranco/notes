package main

var a string
var done bool

type Unit struct{}

func setup(done chan Unit) {
	a = "hello, world"
	done <- Unit{}
}

func main() {
	done := make(chan Unit)
	go setup(done)
	<-done
	print(a)
}
