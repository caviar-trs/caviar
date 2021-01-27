import sys
import re
from Rule import Rule
from Expression import Expression
from Stack import Stack
import pandas as pd


def extract(path):
    txtfile = open(path, 'r')
    remove = ['int32', 'float32', 'select',
              'broadcast', 'ramp', 'fold', 'Overflow']
    rules = []
    for line in txtfile:
        rule = re.search('rewrite\((.*)\) *\|\|$', line)
        if rule:
            formated_rule = [r for r in rule.group(1)]
            formated_rule = ''.join(formated_rule)
            sides, _ = remove_condition(formated_rule)
            sides = ''.join(sides)
            ok = True
            for f in remove:
                if f in sides:
                    ok = False
            if ok:
                rules.append(sides)
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
        elif(rule[i] == ','):
            if stack.top == -1 and additional_comma > 0:
                additional_comma += 1
                sides = ''.join(rule[:i])
                condition = ''.join(rule[i+1:])
            elif stack.top == -1 and additional_comma == 0:
                additional_comma += 1
            else:
                stack.pop()
        i += 1
    if additional_comma == 1:
        sides = ''.join(rule)
        condition = ''
    return (sides, condition)


def main(params):
    rules = extract(params[0])
    # rules = pd.DataFrame(rules)
    for i, rule in enumerate(rules):
        print(str(i + 1) + " " + rule)
        if i+1 == 10:
            break
        # print(Rule(rule).infix_rule())


if __name__ == '__main__':
    main(sys.argv[1:])
