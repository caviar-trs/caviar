import sys
import os
import glob
import json

def read_data(path):
    all_data = []
    for file_name in glob.glob(path + "/*.json"):
        f = open(file_name)
        data = json.load(f)
        all_data.extend(data)
    unique_data = list({d['expression']['start']: d for d in all_data}.values())
    return unique_data



def main(params):
    results_folder = params[0]
    batch_size = int(params[1])
    data = read_data(results_folder)
    data_len = len(data)
    for i in range(data_len // batch_size):
        data_batch = data[i*batch_size:(i+1)*batch_size]
        print("Writing {} expressions into results-batch-{}.json".format(batch_size, i))
        with open("results/results-batch-{}.json".format(i), "w") as outfile:
            outfile.write(json.dumps(data_batch))
    if data_len % batch_size != 0:
        data_batch = data[(data_len // batch_size) * batch_size:]
        print("Writing {} expressions into results-batch-{}.json".format(batch_size, i))
        with open("results/results-batch-{}.json".format(data_len // batch_size), "w") as outfile:
                outfile.write(json.dumps(data_batch))
    

if __name__ == '__main__':
    main(sys.argv[1:])