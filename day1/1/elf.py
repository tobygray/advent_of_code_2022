import sys

elves = []

current_elf = 0
for line in sys.stdin:
    line = line.strip()
    if len(line) == 0:
        elves.append(current_elf)
        current_elf = 0
    else:
        current_elf += int(line)
elves.append(current_elf)

print(max(*elves))
