class Stack:

    def __init__(self, length):
        self.stack = [None for _ in range(length)]
        self.top = -1
        self.length = length

    # Pushing element inside stack
    def push(self, val):
        if self.top == self.length - 1:
            return -999
        self.top += 1
        self.stack[self.top] = val
        return 1

    # Pop element from stack
    def pop(self):
        if self.top == -1:
            return -999
        val = self.stack[self.top]
        self.stack[self.top] = None
        self.top -= 1
        return val