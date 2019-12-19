from input import input
from part_1 import opcode_program

if __name__ == "__main__":
    noun_difference = 248832
    goal = 19690720

    noun = 12
    verb = 2
    output = 0
    while output < goal:
        if (goal - output <= noun_difference):
            verb += 1
        else:
            noun += 1
        input[1] = noun
        input[2] = verb
        output = opcode_program(input)

    answer = 100 * noun + verb
    print(f"Program executed, result: {output}, noun: {noun}, verb: {verb}, answer: {answer}")
