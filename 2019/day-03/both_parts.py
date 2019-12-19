def process_wire(instr_line):
    current_pos = [0, 0]
    for instr in instr_line.split(','):
        for _ in range(int(instr[1:])):
            current_pos[0 if instr[0] in ('L', 'R') else 1] += -1 if instr[0] in ('L', 'D') else 1
            yield tuple(current_pos)

with open('input.txt', 'r') as f:
    wires = [list(process_wire(line)) for line in f.readlines()]

intersections = set(wires[0]) & set(wires[1])
print(min(abs(x)+abs(y) for (x, y) in intersections)) # part 1
print(2 + min(sum(wire.index(intersect) for wire in wires) for intersect in intersections)) # part 2
