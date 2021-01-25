from Stack import Stack
# from QuickSort import quickSort


class Expression:
    def __init__(self, expr: str):
        # self.expr = Expression.fun_to_op(
        #     Expression.expr_str_to_arr(Expression.minus_plus(expr)))
        # self.expr = Expression.fun_to_op(
        #     Expression.expr_str_to_arr(Expression.minus_plus(expr)))
        self.expr = Expression.expr_str_to_arr(Expression.minus_plus(expr))

    def print(self):
        print(self.toString() + "\n")

    def toString(self):
        return '"(' + ' '.join(self.expr) + ')"' if self.expr[0] != '(' else '"' + ' '.join(self.expr) + '"'

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

    @staticmethod
    def partition(arr, low, high):
        i = (low-1)		 # index of smaller element
        pivot = arr[high]	 # pivot

        for j in range(low, high):

            # If current element is bigger than or
            # equal to pivot
            if Expression.priority(arr[j][0]) >= Expression.priority(pivot[0]):

                # increment index of smaller element
                i = i+1
                arr[i], arr[j] = arr[j], arr[i]

        arr[i+1], arr[high] = arr[high], arr[i+1]
        return (i+1)

    # The main function that implements QuickSort
    # arr[] --> Array to be sorted,
    # low --> Starting index,
    # high --> Ending index

    # Function to do Quick sort
    @staticmethod
    def quickSort(arr, low, high):
        if len(arr) == 1:
            return arr
        if low < high:

            # pi is partitioning index, arr[p] is now
            # at right place
            pi = Expression.partition(arr, low, high)

            # Separately sort elements before
            # partition and after partition
            Expression.quickSort(arr, low, pi-1)
            Expression.quickSort(arr, pi+1, high)

    def add_parentheses(self):
        expr = self.expr
        i = 0
        stack = Stack(len(expr))
        while i < len(expr):
            if expr[i] in ('+', '*', '%', '/', '<', '>', '&&', '||', '==', '!=', "<=", ">="):
                stack.push([expr[i], i])
                # if expr[i-1] != '(':
                #     expr.insert(i-1, '(')
            # elif(expr[i] == ','):
            #     expr[i] = stack.pop()
            i += 1
        stack_operations = [i for i in stack.stack if i]
        Expression.quickSort(stack_operations, 0, len(stack_operations)-1)
        print(expr)
        for op_index in range(len(stack_operations)):
            position = stack_operations[op_index][1]
            if expr[position-1] != ")":
                if expr[position-2] != "(":
                    expr.insert(position-1, "(")
            else:
                while expr[position] != "(" and position > 0:
                    position -= 1
                expr.insert(position, "(")

            print("expr: ", expr)
            print("position: ", position)
            # update position of the operation after adding (
            for i in range(len(stack_operations)):
                if stack_operations[i][1] >= position:
                    stack_operations[i][1] += 1

            print(stack_operations)

            position = stack_operations[op_index][1]
            if expr[position+1] != "(":
                if position+2 < len(expr):
                    if expr[position+2] != ")":
                        expr.insert(position+2, ")")
                else:

                    expr.insert(position, ")")
            else:
                while expr[position] != ")" and position < len(expr):
                    position += 1
                expr.insert(position, ")")

            # update position of the operation after adding )
            for i in range(len(stack_operations)):
                if stack_operations[i][1] >= position:
                    stack_operations[i][1] += 1
            position += 1

            print(stack_operations[op_index])

        print(expr)

        return stack.stack


if __name__ == '__main__':
    arr = [i for i in Expression("c1 + x * y + z").add_parentheses() if i]
    print("\n\n")
    arr = [i for i in Expression("(c1 + x) * y + z").add_parentheses() if i]
    print("\n\n")
    arr = [i for i in Expression(
        "x2 + (c1 + x) * y + z").add_parentheses() if i]

# # Driver code to test above
# n = len(arr)
# Expression.quickSort(arr, 0, n-1)
# print("Sorted array is:")
# for i in range(n):
#     print(arr[i]),
