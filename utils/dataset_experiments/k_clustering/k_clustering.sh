#!/bin/bash
echo $2
K=$1
python "`dirname "$0"`/k_clustering.py" $1
../../../target/release/egg_halides_trs test_classes ../../../results/expressions_egg.csv 1000000 100000000 $2 "./results/k_"$K"_classes.json"
