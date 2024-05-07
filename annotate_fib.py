import matplotlib.pyplot as plt
from PIL import Image

def fib(n, F=[0, 1]):
    F.extend(sum(F[-2:])for _ in range(n-len(F)+1)); return F

X_OFFSET = 420
X_SPACING = 70

Y_OFFSET = 70
Y_SPACING = 70

grid = [
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
    ["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
]

img = Image.open('Untitled.png')

fib_row = 4

candidates = []

for f in fib(54):
    f = str(f)
    padding = 11 - len(f)

    for i in range(padding+1):
        candidates.append("_"*i + f + "_"*(padding - i))

def plot_grid(grid, img):    
    plt.imshow(img)
    
    for i in range(11):
        for j in range(11):
            s = grid[i][j]
            plt.text(X_OFFSET + X_SPACING * j, Y_OFFSET + Y_SPACING * i, s)
    
    plt.show(block=False)

target_length = 3
candidates = list(filter(lambda x: len(x.replace("_", "")) == target_length, candidates))

print(candidates)

for idx, candidate in enumerate(candidates):
    print(int(idx / (12 - target_length)), "of", len(candidates) / (12 - target_length))
    print(candidate)
    grid[fib_row] = [*candidate]
    plot_grid(grid, img)
    input()
    plt.close()
