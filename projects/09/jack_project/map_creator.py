import sys

filename = sys.argv[1]
m = []
start_pos = []

def map2int(c, x, y):
    match c:
        case ' ':
            return 0
        case '#':
            return 1
        case 'b':
            return 2
        case 'e':
            return 3
        case 's':
            return -1

print("""let i = 0;
while (i < 512) {
    let m[i] = 0;
    let i = i + 1;
} 
""")
with open(filename) as f:
    for y, line in enumerate(f):
        line = line.strip()
        if len(line) == 0:
            m = m + [0]*32
            continue
        if all(list(map(lambda x: x == line[0], line))):
            print('let i = 0;')
            print('while (i < 32) {')
            print(f'\tlet m[{y} * 32 + i] = {map2int(line[0], -1, -1)};')
            print(f'\tlet i = i + 1;')
            print('}')
            m = m + [0]*32
            continue
        for x, c in enumerate(line):
            i = map2int(c, x, y)
            if i >= 0:
                m.append(i)
            else:
                m.append(0)
                start_pos = [x, y]

print(f"let sp = Position.new({start_pos[0]}, {start_pos[1]});")
for i, c in enumerate(m):
    if c == 0:
        continue
    print(f'let m[{i}] = {c};')