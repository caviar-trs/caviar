from Stack import Stack


# from QuickSort import quickSort


class Expression:
    def __init__(self, expr: str):
        # self.expr = Expression.fun_to_op(
        #     Expression.expr_str_to_arr(Expression.minus_plus(expr)))
        # self.expr = Expression.fun_to_op(
        #     Expression.expr_str_to_arr(Expression.minus_plus(expr)))
        self.expr = Expression.expr_str_to_arr(Expression.minus_plus(expr))
        Expression.add_parentheses(self)

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
            elif s[i:i + 2] in ('&&', '||', '==', '!='):
                res.append(s[i:i + 2])
                i += 1
            elif s[i] in ('+', '*', '%', '/', '(', ')', ',', '!'):
                res.append(s[i])
            elif s[i] == '-':
                if s[i + 1].isdigit():
                    n = s[i]
                    while i + 1 < len(s) and s[i + 1].isdigit():
                        i += 1
                        n += s[i]
                    res.append(n)
                else:
                    res.append(s[i])
            elif s[i] in ('<', '>'):
                if s[i + 1] in ('='):
                    res.append(s[i] + s[i + 1])
                    i += 1
                else:
                    res.append(s[i])
            elif s[i].isdigit():
                n = s[i]
                while i + 1 < len(s) and s[i + 1].isdigit():
                    i += 1
                    n += s[i]
                res.append(n)
            elif s[i:i + 3] in ('max', 'min'):
                res.append(s[i:i + 3])
                i += 2
            elif s[i:i + 6] == 'select':
                res.append(s[i:i + 6])
                i += 5
            elif s[i].isalpha():
                n = s[i]
                while (i + 1 < len(s)) and (s[i + 1].isdigit() or s[i + 1].isalpha()):
                    i += 1
                    n += s[i]
                res.append(n)
            i += 1
        return res

    def minus_plus(s):
        return s.replace('+ -', '- ')

    def remove_space(s):
        return s.replace(' ', '')

    @staticmethod
    def fun_to_op(expr):
        i = 0
        stack = Stack(len(expr))
        while i < len(expr):
            if expr[i] in ('min', 'max'):
                stack.push(expr[i])
                del expr[i]
                i -= 1
            elif expr[i] == ',':
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
        expr = Expression.fun_to_op(self.expr)
        prefix = []
        revInfix = []

        # Replacing '(' with ')' and
        # reversing the input string
        for i in range(len(expr) - 1, -1, -1):
            ch = expr[i]

            if ch == '(':
                ch = ')'
            elif ch == ')':
                ch = '('

            revInfix.append(ch)
        # print(revInfix)
        expr = revInfix

        # Declaration of stack
        stack = Stack(len(expr))

        for i in expr:
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
        expr = prefix[::-1]
        return expr

    @staticmethod
    def partition(arr, low, high):
        i = (low - 1)  # index of smaller element
        pivot = arr[high]  # pivot

        for j in range(low, high):

            # If current element is bigger than or
            # equal to pivot
            if Expression.priority(arr[j][0]) >= Expression.priority(pivot[0]):
                # increment index of smaller element
                i = i + 1
                arr[i], arr[j] = arr[j], arr[i]

        arr[i + 1], arr[high] = arr[high], arr[i + 1]
        return (i + 1)

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
            Expression.quickSort(arr, low, pi - 1)
            Expression.quickSort(arr, pi + 1, high)

    def add_parentheses(self):
        expr = self.expr
        i = 0
        list_operations = []
        while i < len(expr):
            if expr[i] in ('+', '-', '*', '%', '/', '<', '>', '&&', '||', '==', '!=', "<=", ">="):
                list_operations.append([expr[i], i])
                # if expr[i-1] != '(':
                #     expr.insert(i-1, '(')
            # elif(expr[i] == ','):
            #     expr[i] = stack.pop()
            i += 1
        Expression.quickSort(list_operations, 0, len(list_operations) - 1)
        for op_index in range(len(list_operations)):
            position = list_operations[op_index][1]
            right_inserted = False
            if expr[position - 1] != ")":
                if expr[position - 2] != "(":
                    position -= 1
                    expr.insert(position, "(")
                    right_inserted = True
                elif expr[position - 3] in ["min", "max"]:
                    position -= 1
                    expr.insert(position, "(")
                    right_inserted = True
            else:
                paren_stack = Stack(len(expr))
                position -= 1
                while True:
                    if expr[position] == ")":
                        paren_stack.push(")")
                    elif expr[position] == "(":
                        paren_stack.pop()
                    position -= 1
                    if paren_stack.top < 0:
                        break
                if expr[position] in ["min", "max"]:
                    position -= 1
                position += 1
                expr.insert(position, "(")
                right_inserted = True

            # update position of the operation after adding (
            if right_inserted:
                for i in range(len(list_operations)):
                    if list_operations[i][1] >= position:
                        list_operations[i][1] += 1

            position = list_operations[op_index][1]
            left_inserted = False
            if expr[position + 1] != "(":
                if position + 2 < len(expr):
                    if expr[position + 2] != ")":
                        position += 2
                        expr.insert(position, ")")
                        left_inserted = True
                else:
                    position += 2
                    expr.insert(position, ")")
                    left_inserted = True
            else:
                paren_stack = Stack(len(expr))
                position += 1
                while True:
                    if expr[position] == "(":
                        paren_stack.push("(")
                    elif expr[position] == ")":
                        paren_stack.pop()
                    position += 1
                    if paren_stack.top < 0 or position == len(expr):
                        break
                expr.insert(position, ")")
                left_inserted = True

            # update position of the operation after adding (
            if left_inserted:
                for i in range(len(list_operations)):
                    if list_operations[i][1] >= position:
                        list_operations[i][1] += 1

        return expr


if __name__ == '__main__':
    # arr = [i for i in Expression("c1 + x * y + z").add_parentheses() if i]
    arr = Expression("( min ( ( y - z ) , x ) + z )")
    #                 ( min ( ( y - z ) , x ) + z )
    print(' '.join(arr.infixToPrefix()))
    # arr = [i for i in Expression(
    #     "x2 + (c1 + x) * (y + z)").add_parentheses() if i]

# # Driver code to test above
# n = len(arr)
# Expression.quickSort(arr, 0, n-1)
# print("Sorted array is:")
# for i in range(n):
#     print(arr[i]),
