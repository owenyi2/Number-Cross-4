records = []

start = "START"

with open("temp.csv", "r") as f:
    records = [row.rstrip().split(",") for row in f.readlines()]

levels = { i: list(filter(lambda x: x[2] == str(i), records)) for i in range(10) }
levels[-1] = [[start, x, "-1"] for x in set([y[0] for y in levels[0]])]

# levels = {
#     -1: [[start, "A", -1]],
#     0: [["A", "B", 0], ["A", "C", 0]],
#     1: [["B", "E", 1], ["C", "D", 1]],
#     2: [["E", "F", 2], ["D", "G", 2]],
# }

class Traveler:
    def __init__(self, levels):
        self.paths = []
        self.graph = levels
        self.nodes()
        self.parent_map = {}

    def nodes(self):
        self.nodes = dict()
        self.nodes[start] = False
        for level in self.graph.values():
            for row in level:
                self.nodes[row[0]] = False
                self.nodes[row[1]] = False
            
    def run(self):
        S = [(start, -1)]
        while S: 
            v, level = S.pop()
            if level == 10:
                return v

            if not self.nodes[v]:
                self.nodes[v] = True
                for edge in self.graph.get(level, []):
                    if edge[0] == v:
                        S.append((edge[1], level+1))
                        self.parent_map[edge[1]] = edge[0]

traveler = Traveler(levels)
target = traveler.run()
traveler.parent_map

curr = target 
while (curr != None):
    print(curr)
    curr = traveler.parent_map[curr]
