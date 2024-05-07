import matplotlib.pyplot as plt
from PIL import Image

X_OFFSET = 420
X_SPACING = 70

Y_OFFSET = 70
Y_SPACING = 70

def pad(s, i):
    return i * "_" + s + (11-len(s) - i) * "_"

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

def plot_grid(grid, img):    
    plt.imshow(img)
    
    for i in range(11):
        for j in range(11):
            s = grid[i][j]
            plt.text(X_OFFSET + X_SPACING * j, Y_OFFSET + Y_SPACING * i, s)
    
    plt.show()

img = Image.open('Untitled.png')

def workspace_1():
    fib_row = 4
    grid[fib_row] = [*pad("17711", 1)]
     

    plot_grid(grid, img)

workspace_1()
