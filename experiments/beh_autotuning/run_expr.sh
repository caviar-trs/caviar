#!/bin/bash
for i in 0.01 0.05 0.1 0.25 0.5 0.75 1
do
     ../../target/release/egg_halides_trs prove_exprs_passes ../../data/prefix/expressions_egg.csv 10000000 10000000 3 $i
done
