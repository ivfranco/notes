#! /bin/bash

MIT_DIR=$(dirname "$0")
NOTE_DIR=$(cat "$MIT_DIR/NOTE_DIR")
echo "copying to $NOTE_DIR"

files=(
    "copy.sh"
    # lab 1
    "src/mr/coordinator.go"
    "src/mr/rpc.go"
    "src/mr/worker.go"
    # lab 2
    "src/mrapps/grep.go"
    "src/raft/raft.go"
    "src/raft/test.sh"
)

for file in "${files[@]}"; do
    path="$MIT_DIR/$file"
    if test -f "$path"; then
        mkdir -p $(dirname "$NOTE_DIR/$file")
        cp "$path" "$NOTE_DIR/$file"
        echo "copied $file"
    else
        >&2 echo "$file not found"
        exit 1
    fi
done
