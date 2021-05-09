#!/bin/bash

for i in 55 60 65 70 75
do
	echo "Starting: K="$i
	python "`dirname "$0"`/../k_clustering.py" $i "`dirname "$0"`/5k_dataset.json"
	../../../../target/release/egg_halides_trs test_classes ../../../../data/prefix/expressions_egg.csv 1000000 100000000 3 "./results/k_"$i"_classes.json" 10
done


