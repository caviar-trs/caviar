#!/bin/bash
python3 ./utils/infix-to-prefix/Extractor.py "$1"
cargo run --release ./results/rules_egg.csv