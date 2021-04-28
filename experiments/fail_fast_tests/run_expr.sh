#!/bin/bash
python utils/infix-to-prefix/Expression.py "$1" > tmp/exprs.txt
cargo run --release 100000000 100000000 3