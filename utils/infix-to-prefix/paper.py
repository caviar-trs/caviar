import os
import sys
import csv
from Expression import Expression


def main(params):
    remove = ['int32', 'float32', 'select',
              'broadcast', 'ramp', 'fold',
              'Overflow', 'can_prove', 'canprove'
              'op->type', 'op->type', 'Call', 'this', 'IRMatcher', 'likely_if_innermost', 'let']
    results = []
    exprs = []
    with open(params[0], 'r') as f:
        for line in f:
            expr = line.strip()
            expr = expr.replace("(uint1)", "")
            expr = expr.replace("(uint64)", "")
            expr = expr.replace("(uint8)", "")
            expr = expr.replace("(uint16)", "")
            if "uint" in expr:
                continue
            if expr in exprs:
                continue
            ok = True
            for tabou in remove:
                if tabou in expr:
                    ok = False
            if ok:
                expr_temp = Expression(expr)
                prefix = " ".join(expr_temp.infixToPrefix())
                results.append([expr, prefix])
                exprs.append(expr)
    with open('results/infix_prefix.csv', 'w') as w:
        # using csv.writer method from CSV package
        write = csv.writer(w, delimiter=";")
        write.writerow(['infix', 'prefix'])
        write.writerows(results)


if __name__ == '__main__':
    main(sys.argv[1:])
