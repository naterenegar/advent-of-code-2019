#!/usr/bin/env python3

def calculate_fuel(fuel):
	new_fuel = int(fuel/3) - 2 
	if(new_fuel > 0):
		return new_fuel + calculate_fuel(new_fuel) 
	else:
		return 0 


sum = 0
with open('input.txt', 'r') as puzzle_input:
	for line in puzzle_input:
		sum = sum + calculate_fuel(int(line)) 
print(sum)
