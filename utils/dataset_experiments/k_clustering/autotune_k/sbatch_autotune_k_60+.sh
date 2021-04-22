#!/bin/bash
#
#SBATCH --job-name=testing_back_integer
#SBATCH --output=back_integer.txt
#
#SBATCH --ntasks=48

for i in 60 65 70 75 80 85 90 95 100 105 111
do
	echo "Starting: K="$i
	python "`dirname "$0"`/../k_clustering.py" $i "`dirname "$0"`/5k_dataset.json"
	../../../../target/release/egg_halides_trs test_classes ../../../../data/prefix/expressions_egg.csv 1000000 100000000 3 "./results/k_"$i"_classes.json"
done

