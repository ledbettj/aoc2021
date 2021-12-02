#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "usage: $0 <day>"
    exit 1
fi

DAY="$1"
SRC="src/day$DAY.rs"

if [ -f "$SRC" ]; then
    echo "$SRC exists!"
    exit 1
fi

echo "#[allow(dead_code)]" >> src/lib.rs
echo "mod day$DAY;" >> src/lib.rs
cat .template | sed s/DAY/$DAY/g > "$SRC"

if [ -f ".session-cookie" ]; then
    curl -b session=$(cat .session-cookie) \
         https://adventofcode.com/2021/day/$DAY/input > "inputs/day$DAY.txt"

    echo "Fetched input!"
fi

echo "done!"
