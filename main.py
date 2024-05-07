records = []

with open("37.txt") as f:
    records = [line.rstrip().split(" ") for line in f.readlines()]

numbers = {
    i: list(map(lambda x: x[0], filter(lambda x: x[1] == str(i), records))) for i in range(2, 12)
}

class rec:
    def __init__(self, numbers):
        self.strings = []
        self.numbers = numbers
        self.string_length = 11

    def rec(self, string):
        candidate_lengths = range(2, self.string_length - len(string) + 1)
        
        if candidate_lengths:
            for length in candidate_lengths:
                for num in self.numbers[length]:
                    next_string = string + str(num)
                    if len(next_string) == self.string_length:
                        self.rec(next_string)
                    else:
                        self.rec(next_string + "*")

        else:
            if len(string) == self.string_length:
                self.strings.append(string)
            return

a = rec(numbers)

a.rec("")
print(len(a.strings))

