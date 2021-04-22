import pandas as pd
import math
import json
import sys

if __name__ == "__main__":
    k = int(sys.argv[1])
    df = pd.read_json(sys.argv[2], orient=str)
    df = df.join(df['expression'].apply(pd.Series))
    df = df.drop(columns="expression")
    df1 = df.join(df['rules'].apply(lambda x: pd.Series(1, index=x)).fillna(0))
    df1 = df1.drop(columns='rules')
    test = df1.sum(axis=0, numeric_only=True)

    sorted_rules = test.sort_values(ascending=False)
    # print(sorted_rules.index[0])

    limit = math.ceil(len(sorted_rules)/k)
    classes = []
    for j in range(limit):
        i = 0
        cur_class = []
        while (i < k*(j+1) and i < len(sorted_rules)):
            cur_class.append(sorted_rules.index[i])
            i += 1
        classes.append(cur_class)
    print(classes, "\n\n", f"Total Classes: {len(classes)}")

    with open(f"./results/k_{k}_classes.json", 'w') as outfile:
        json.dump(classes, outfile)
