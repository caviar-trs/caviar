import sys
import re
from Rule import Rule
from Expression import Expression
from Stack import Stack
import csv


def extract(path):
    txtfile = open(path, 'r')
    remove = ['int32', 'float32', 'select',
              'broadcast', 'ramp', 'fold', 
              'Overflow', 'can_prove', 'canprove'
              'op->type', 'Call', 'this']
    rules = []
    for line in txtfile:
        rule = re.search('rewrite\((.*)\) *\|\|$', line)
        if rule:
            formated_rule = [r for r in rule.group(1)]
            formated_rule = ''.join(formated_rule)
            ok = True
            for f in remove:
                if f in formated_rule:
                    ok = False
            if ok:
                sides, condition = remove_condition(formated_rule)
                sides = ''.join(sides)
                rules.append([sides, condition if condition else " "])
    return rules


def remove_condition(rule):
    rule = Expression.expr_str_to_arr(Expression.minus_plus(rule))
    sides = []
    condition = []
    i = 0
    stack = Stack(len(rule))
    additional_comma = 0
    while i < len(rule):
        if rule[i] in ('min', 'max'):
            stack.push(rule[i])
        elif (rule[i] == ','):
            if stack.top == -1 and additional_comma > 0:
                additional_comma += 1
                sides = ''.join(rule[:i])
                condition = ''.join(rule[i + 1:])
            elif stack.top == -1 and additional_comma == 0:
                additional_comma += 1
            else:
                stack.pop()
        i += 1
    if additional_comma == 1:
        sides = ''.join(rule)
        condition = ''
    return (sides, condition)


def extract_min_max_params(expr):
    expr = Expression.expr_str_to_arr(Expression.minus_plus(expr))
    left = ""
    right = ""
    i = 2
    stack = Stack(len(expr))
    while i < len(expr):
            if expr[i] in ('min', 'max'):
                stack.push(expr[i])
            elif(expr[i] == ','):
                if stack.top == -1:
                    left = ' '.join(expr[2:i]) 
                    right = ' '.join(expr[i+1:len(expr)-1]) 
                else:
                    stack.pop()
            i += 1
    return left, right


def main(params):
    rules = extract(params[0])
    rules_trs = []
    for i, rule in enumerate(rules):
        rul = Rule(rule[0])
        rules_trs.append([i+1, rul.toString(), *rul.infix_rule(), rule[1]])
    #print(rules_trs)
    with open('results/rules_egg.csv', 'w') as f:
        # using csv.writer method from CSV package
        write = csv.writer(f)
        write.writerow(['index', 'rule', 'LHS', 'RHS', 'Condition'])
        write.writerows(rules_trs)


if __name__ == '__main__':
    main(sys.argv[1:])
    # l, r = extract_min_max_params(sys.argv[1])
    # print(l)
    # print(r)
