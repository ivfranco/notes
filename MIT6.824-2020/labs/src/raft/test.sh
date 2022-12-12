#!/bin/bash

repeat=$1
filter=$2

for (( i=1; i<=$1; i++ )); do
    if go test -run $filter -race > "$2.log"; then
        echo "Passed iteration $i"
    else
        echo "Failed iteration $i"
        exit 1
    fi
done
