from Stack import Stack
from Expression import Expression
import re

class Rule:
    def __init__(self, rule: str):
        (self.left_side, self.right_side) = self.extract_sides(
            Expression.expr_str_to_arr(Expression.minus_plus(rule))
        )

        if "(a)" in self.right_side:
            # print(self.left_side)
            self.right_side = Rule.extract_min_max_params(self.left_side)[1]
        if "(b)" in self.right_side:
            self.right_side = Rule.extract_min_max_params(self.left_side)[0]
        # elif "(b)" in right:
        #     right = Rule.extract_min_max_params(left)[0]

    def print(self):
        print(self.toString())

    def toString(self):
        return self.left_side + " => " + self.right_side

    def extract_sides(self, rule):
        left = ""
        right = ""
        i = 0
        stack = Stack(len(rule))
        # print(rule)
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
        s_right = ' '.join(right)
        s_left = ' '.join(left)

        # Replacing ( - x ) with ( * x -1 )
        s_left = re.sub("\( \- (?P<var>[a-zA-Z_$][a-zA-Z_$0-9]*) \)", r'(* \1 -1)', s_left)
        s_right = re.sub("\( \- (?P<var>[a-zA-Z_$][a-zA-Z_$0-9]*) \)", r'(* \1 -1)', s_right)

        # Replacing ( + x ) with x
        # s_left = re.sub("\( \+ (?P<var>[a-zA-Z_$][a-zA-Z_$0-9]*) \)", r' \1 ', s_left)
        # s_right = re.sub("\( \+ (?P<var>[a-zA-Z_$][a-zA-Z_$0-9]*) \)", r' \1 ', s_right)

        # print(' '.join(left), ' '.join(right))
        return s_left, s_right

    @staticmethod
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
                        left = ' '.join(expr[3:i])
                        right = ' '.join(expr[i+1:len(expr)-2])
                    else:
                        stack.pop()
                i += 1
        return left, right



if __name__ == '__main__':
    rule = Rule('max(x*c0, y) + (x*c1), max((x*c1) + y, 0)')
    # rule.print()
    print(rule.infix_rule())
