from input import input

def opcode_program(input):
    program = input.copy()
    instruction = 0
    execute = True
    while execute:
        opcode = program[instruction]
        if opcode == 1:
            in1 = program[instruction + 1]
            in2 = program[instruction + 2]
            out = program[instruction + 3]
            program[out] = program[in1] + program[in2]
            instruction += 4
        elif opcode == 2:
            in1 = program[instruction + 1]
            in2 = program[instruction + 2]
            out = program[instruction + 3]
            program[out] = program[in1] * program[in2]
            instruction += 4
        elif opcode == 99:
            execute = False
            return program[0]
        else:
            raise Exception(f"Unknown opcode encountered, instruction: {instruction}, opcode: {opcode}")

if __name__ == "__main__":
    input[1] = 12
    input[2] = 2

    output = opcode_program(input)

    print(f"Program executed, result: {output}")
