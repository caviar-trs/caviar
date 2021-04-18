#!/bin/bash
#
#SBATCH --job-name=generating_dataset
#SBATCH --output=output.txt
#SBATCH --partition=lanka-v3
#SBATCH --ntasks=1

srun ../../../target/release/egg_halides_trs dataset ../../../data/prefix/expressions_egg.csv 10000000 100000 3
