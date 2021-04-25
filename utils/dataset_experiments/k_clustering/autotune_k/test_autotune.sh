#!/bin/bash
#
#SBATCH --job-name=testing_back_integer
#SBATCH --output=back_integer.txt
#
#SBATCH --ntasks=48

for i in 10 35 55 75 111
do
	printf "\n\nStarting: K="$i"\n"
#	python "`dirname "$0"`/../k_clustering.py" $i "`dirname "$0"`/5k_dataset.json"
	../../../../target/release/egg_halides_trs test_classes ../../../../data/prefix/test_expressions_egg.csv 3 100000000 3 "./results/k_"$i"_classes.json"
done

