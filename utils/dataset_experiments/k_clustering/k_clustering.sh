#!/bin/bash
K=$1
python "`dirname "$0"`/k_clustering.py" $1
RUST_BACKTRACE=1 cargo run --release test_classes results/expressions_egg.csv 1000 100000 3 "./results/k_"$K"_classes.json"
