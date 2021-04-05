import os
results = []
for file in os.listdir("results"):
    if file.endswith(".csv"):
        df = pd.read_csv(f'results/{file}')
        true_df = df.loc[df['result'] == True]
        true_df.head()

        result = pd.DataFrame()
        result = true_df[['index', 'class']].groupby(['class']).count()
        result.columns = ['Frequency']
        result = result.sort_values(by=['Frequency'], ascending=False)
        summ = result.sum()
        result['percentage'] = result.apply(lambda row: row/summ*100.0, axis=1)
        # print(file)
        # print(result)
        results.append([file, result, true_df['total_time'].sum()])
        # print("\n------------------\n")

print(results[1][2]/results[0][2])
