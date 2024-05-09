import re
import sys

print_row = int(sys.argv[1])

row = 0
rows = {}

row_number = re.compile(r"^[0-9]|[1-9][0-9]*$")

with open("results.txt") as f:
    for line in f.readlines():
        if row_number.match(line) is None: 
            try:
                rows[row].append(line.rstrip())
            except KeyError:
                rows[row] = [line.rstrip()]
        else:
            row = int(line.rstrip())

print(rows[print_row])
