from Stack import Stack
from Expression import Expression


class Rule:
    def __init__(self, rule: str):
        (self.left_side, self.right_side) = self.extract_sides(
            Expression.expr_str_to_arr(Expression.minus_plus(rule))
        )

    def print(self):
        print(self.toString())

    def toString(self):
        return self.left_side + " => " + self.right_side

    def extract_sides(self, rule):
        left = ""
        right = ""
        i = 0
        stack = Stack(len(rule))
        print(rule)
        while i < len(rule):
            if rule[i] in ('min', 'max'):
                stack.push(rule[i])
            elif(rule[i] == ','):
                if stack.top == -1:
                    left = "(" + ' '.join(rule[:i]) + ")"
                    right = "(" + ' '.join(rule[i+1:]) + ")"
                else:
                    stack.pop()
            i += 1
        return left, right

    def infix_rule(self):
        left = Expression(self.left_side)
        left = left.infixToPrefix()
        right = Expression(self.right_side)
        right = right.infixToPrefix()
        print(' '.join(left), ' '.join(right))
        return ' '.join(left), ' '.join(right)


if __name__ == '__main__':
    rule = Rule('max(x*c0, y) + (x*c1), max((x*c1) + y, 0)')
    # rule.print()
    print(rule.infix_rule())
