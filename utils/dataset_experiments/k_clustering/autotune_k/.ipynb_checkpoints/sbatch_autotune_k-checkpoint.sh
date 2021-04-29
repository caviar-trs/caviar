#!/bin/bash
#
#SBATCH --job-name=testing_back_integer
#SBATCH --output=back_integer.txt
#
#SBATCH --ntasks=48

for i in 5 10 15 20 25 30 35 40 45 50 55
do
	echo "Starting: K="$i
	python "`dirname "$0"`/../k_clustering.py" $i "`dirname "$0"`/new_dataset.json"
	../../../../target/release/egg_halides_trs test_classes ../../../../data/prefix/expressions_egg.csv 1000000 100000000 3 "./results/k_"$i"_classes.json"
done
