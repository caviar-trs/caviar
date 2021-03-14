import sys
import re
from Rule import Rule
from Expression import Expression
from Stack import Stack
import csv


def extract(path):
    print(path)
    with open(path) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',')
        remove = ['int32', 'float32', 'select',
                  'broadcast', 'ramp', 'fold',
                  'Overflow', 'can_prove', 'canprove'
                  'op->type', 'op->type', 'Call', 'this', 'IRMatcher']
        exprs = []

        for i, row in enumerate(csv_reader):
            next_expr = False
            for tabou in remove:
                if tabou in row[0]:
                    # print("=====", tabou)
                    next_expr = True
            if next_expr:
                # print("Skipped row :", i)
                continue

            right = Expression(row[0])
            expr = ' '.join(right.infixToPrefix())
            expr = re.sub("\( \- (?P<var>[a-zA-Z_$][a-zA-Z_$0-9]*) \)", r'(* \1 -1)', expr)
            print(expr)
            exprs.append(expr)
            # if rule:
            #     formated_rule = [r for r in rule.group(1)]
            #     formated_rule = ''.join(formated_rule)
            #     ok = True
            #     for f in remove:
            #         if f in formated_rule:
            #             ok = False
            #     if ok:
            #         sides, condition = remove_condition(formated_rule)
            #         sides = ''.join(sides)
            #         rules.append([sides, condition if condition else " "])
    return exprs


if __name__ == '__main__':
    # arr = [i for i in Expression("c1 + x * y + z").add_parentheses() if i]
    exprs = extract(sys.argv[1])
    frmt = []
    for i, expr in enumerate(exprs):
        frmt.append([i+1, expr])
    # for i, rule in enumerate(rules):
    #     rul = Rule(rule[0])
    #     rules_trs.append([i+1, rul.toString(), *rul.infix_rule(), rule[1]])
    # print(rules_trs)
    with open('results/expressions_egg.csv', 'w') as f:
        # using csv.writer method from CSV package
        write = csv.writer(f)
        write.writerow(["ID", 'Expression'])
        write.writerows(frmt)
