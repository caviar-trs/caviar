from Stack import Stack

class Expression:
    def __init__(self, expr: str):
        self.expr = Expression.fun_to_op(Expression.expr_str_to_arr(Expression.minus_plus(expr)))

    def print(self):
        print('"(' + ' '.join(self.expr) + '")\n')

    def expr_str_to_arr(s):
        res = []
        i = 0
        while i < len(s):
            if s[i] == ' ':
                pass
            elif s[i:i+2] in ('&&', '||', '==', '!='):
                res.append(s[i:i+2])
                i += 1
            elif s[i] in ('+', '*', '%', '/', '(', ')', ',', '!'):
                res.append(s[i])
            elif s[i] == '-':
                if s[i+1].isdigit():
                    n = s[i]
                    while i+1 < len(s) and s[i+1].isdigit():
                        i += 1
                        n += s[i]
                    res.append(n)
                else:
                    res.append(s[i])
            elif s[i] in ('<', '>'):
                if s[i+1] in ('='):
                    res.append(s[i]+s[i+1])
                    i += 1
                else:
                    res.append(s[i])
            elif s[i].isdigit():
                n = s[i]
                while i+1 < len(s) and s[i+1].isdigit():
                    i += 1
                    n += s[i]
                res.append(n)
            elif s[i:i+3] in ('max', 'min'):
                res.append(s[i:i+3])
                i += 2
            elif s[i:i+6] == 'select':
                res.append(s[i:i+6])
                i += 5
            elif s[i].isalpha():
                n = s[i]
                while (i+1 < len(s)) and (s[i+1].isdigit() or s[i+1].isalpha()):
                    i += 1
                    n += s[i]
                res.append(n)
            i += 1
        return res

    def minus_plus(s):
        return s.replace('+ -', '- ')


    def remove_space(s):
        return s.replace(' ', '')


    def fun_to_op(expr):
        i = 0
        stack = Stack(len(expr))
        while i < len(expr):
            if expr[i] in ('min', 'max'):
                stack.push(expr[i])
                del expr[i]
                i -= 1
            elif(expr[i] == ','):
                expr[i] = stack.pop()
            i += 1
        return expr
    
    def priority(opr):
        if opr == '||':
            return 1
        elif opr == '&&':
            return 2
        elif opr == '==' or opr == '!=':
            return 3
        elif opr == '<' or opr == '>' or opr == '<=' or opr == '>=':
            return 4
        elif opr == '+' or opr == '-':
            return 5
        elif opr == '*' or opr == '/' or opr == '%':
            return 6
        elif opr == 'min' or opr == 'max':
            return 7
        elif opr == '!':
            return 8
        return 0

    def isOperator(char):

        return char in ('+', '-', '*', '/', '%', '<', '>', '<=', '>=', 'max', 'min', '&&', '||', '!', '==', '!=')


    def isVar(s):

        return s[0].isalpha and s not in ('max', 'min', 'select')

    def infixToPrefix(self):

        prefix = []
        revInfix = []

        # Replacing '(' with ')' and
        # reversing the input string
        for i in range(len(self.expr)-1, -1, -1):
            ch = self.expr[i]

            if ch == '(':
                ch = ')'
            elif ch == ')':
                ch = '('

            revInfix.append(ch)
        # print(revInfix)
        self.expr = revInfix

        # Declaration of stack
        stack = Stack(len(self.expr))

        for i in self.expr:
            # If character is '(' push it to stack
            if i == '(':
                stack.push(i)
                prefix.append(')')

            # if character is ')' pop out elements
            # from stack until '(' is found or
            # the stack becomes empty
            elif i == ')':
                ch = stack.stack[stack.top]
                while stack.top > -1 and ch != '(':
                    prefix.append(stack.pop())
                    ch = stack.stack[stack.top]
                stack.pop()
                prefix.append('(')

            # if the character is a digit or alphabet
            # add it to the answer string
            elif not Expression.isOperator(i):
                prefix.append(i)

            # if the character is any operator
            # pop out elements from stack until
            # the stack is empty or a symbol with
            # a higher precedence is found in the stack
            elif Expression.isOperator(i):
                ch = stack.stack[stack.top]
                while stack.top > -1 and Expression.priority(i) <= Expression.priority(ch):
                    prefix.append(stack.pop())
                    ch = stack.stack[stack.top]
                stack.push(i)

        # Pop out all remaning elements in
        # the stack and add it to answer string
        while stack.top > -1:
            prefix.append(stack.pop())

        # Return the reversed answer string
        self.expr = prefix[::-1]

