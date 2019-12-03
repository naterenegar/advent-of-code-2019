#!/usr/bin/env python3

sum = 0
with open('input.txt', 'r') as puzzle_input:
	for line in puzzle_input:
		sum = sum + int(int(line) / 3) - 2
print(sum)
