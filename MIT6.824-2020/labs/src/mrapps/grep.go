package main

import (
	"bufio"
	"fmt"
	"regexp"
	"strings"

	"6.824/mr"
)

var pattern regexp.Regexp = *regexp.MustCompile(`\bABOUT\b`)

func Map(filename string, contents string) []mr.KeyValue {
	kva := []mr.KeyValue{}

	lines := bufio.NewScanner(strings.NewReader(contents))
	for lines.Scan() {
		line := lines.Text()
		if pattern.MatchString(line) {
			kva = append(kva, mr.KeyValue{
				Key:   filename,
				Value: line,
			})
		}
	}

	return kva
}

func Reduce(key string, values []string) string {
	builder := &strings.Builder{}
	for _, line := range values {
		fmt.Fprintf(builder, "\n%s", line)
	}
	fmt.Fprintf(builder, "\n")
	return builder.String()
}
