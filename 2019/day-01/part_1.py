from input import input

total_fuel = 0

for mass in input:
   total_fuel += mass // 3 - 2

print(f"Total Fuel: {total_fuel}")
