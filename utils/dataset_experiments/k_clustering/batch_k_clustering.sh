for i in 1 2 3 4 5
do
   ../../../target/release/egg_halides_trs test_classes ../../../results/expressions_egg.csv 1000000 100000000 3 "./results/k_45_classes.json"
    ../../../target/release/egg_halides_trs test_classes ../../../results/expressions_egg.csv 1000000 100000000 3 "./results/k_111_classes.json"
done

