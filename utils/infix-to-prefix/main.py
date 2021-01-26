import sys
import pandas as pd
from Expression import Expression
from Rule import Rule
from Extractor import Extractor

if __name__ == '__main__':
    e = Expression('min(y - z, x) + z')
    e = e.infixToPrefix()
    
    '''
    data_path = sys.argv[1]
    df = pd.read_csv(data_path)
    i = 0
    with open("results.csv", "w") as result:
        for line in df.iterrows():
            print(i)
            i += 1
            line = line[1].values[0]
            expr_s = line[1:-1]
            if 'select' in expr_s or 'float32' in expr_s or 'int32' in expr_s:
                continue
            expr = minus_plus(expr_s)
            expr = expr_str_to_arr(expr)
            expr = fun_to_op(expr)
            # print(' '.join(expr))
            infix = infixToPrefix(expr)
            # print(' '.join(infix))
            result.write(
                '"(' + ' '.join(infix) + '")\n'
            )
    '''
