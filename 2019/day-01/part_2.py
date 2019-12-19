from input import input

def calculate_fuel(mass):
    fuel = mass // 3 - 2
    if fuel <= 0:
        return 0
    fuel += calculate_fuel(fuel)
    return fuel

total_fuel = 0
for mass in input:
    total_fuel += calculate_fuel(mass)

print(f"Part 2 - Total Fuel: {total_fuel}")
