import sys
import re
from Rule import Rule
from Expression import Expression
from Stack import Stack
import csv


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


def main(params):
    rules = extract(params[0])
    rules_trs = []
    for i, rule in enumerate(rules):
        rul = Rule(rule)
        rules_trs.append([i+1, rul.toString(), *rul.infix_rule()])
    print(rules_trs)
    with open('rules_egg.csv', 'w') as f:
        # using csv.writer method from CSV package
        write = csv.writer(f)
        write.writerow(['index', 'rule', 'LHS', 'RHS'])
        write.writerows(rules_trs)


if __name__ == '__main__':
    main(sys.argv[1:])
