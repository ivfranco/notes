#!/bin/bash

EXEC=$1;
REPEAT=5000;

for ((p = 0; p <= 10; p++)); do
    num_pages=$((2 ** $p));
    ns=$($EXEC $num_pages $REPEAT);
    echo "accessing $num_pages pages, $ns ns per access"; 
done

