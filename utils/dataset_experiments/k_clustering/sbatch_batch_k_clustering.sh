#!/bin/bash
#
#SBATCH --job-name=testing_back_integer
#SBATCH --output=back_integer.txt
#
#SBATCH --ntasks=1

for i in 1 2 3 4 5
do
   srun ../../../target/release/egg_halides_trs test_classes ../../../results/expressions_egg.csv 1000000 100000000 3 "./results/k_45_classes.json"
#   srun ../../../target/release/egg_halides_trs test_classes ../../../results/expressions_egg.csv 1000000 100000000 3 "./results/k_111_classes.json"
done

