#!/bin/bash
../../target/release/egg_halides_trs prove_exprs_fast_passes ../../data/prefix/expressions_egg.csv 10000000 10000000 3 0.1
../../target/release/egg_halides_trs prove_exprs_fast ../../data/prefix/expressions_egg.csv 10000000 10000000 3
