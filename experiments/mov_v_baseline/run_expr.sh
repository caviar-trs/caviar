#!/bin/bash
../../target/release/egg_halides_trs prove_exprs_fast_passes ../../data/prefix/evaluation.csv 10000000 10000000 3 0.1
../../target/release/egg_halides_trs prove_exprs_fast_passes ../../data/prefix/evaluation.csv 10000000 10000000 3 0.25
../../target/release/egg_halides_trs prove_exprs ../../data/prefix/evaluation.csv 10000000 10000000 3
