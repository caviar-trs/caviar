data=$1
iter_limit=$2
node_limit=$3
time_limit=$4
reorder=$5
batch_size=$6
continue_from=$7
cores=$8

while true; 
do 
    ./target/release/egg_halides_trs "dataset" $data $iter_limit $node_limit $time_limit $reorder $batch_size $continue_from $cores && break; 
    results_files=$(ls -1q ./results/*.json | wc -l)
    continue_from=$(( ($results_files-1)*$batch_size ))
    echo "Program Failed! Restarting from:"
    echo $continue_from
done