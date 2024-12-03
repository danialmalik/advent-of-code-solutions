import os
import re


def main():
    script_dir = os.path.dirname(__file__)

    sum_p1 = 0
    sum_p2 = 0

    enable_mul = True

    with open(f"{script_dir}/input.txt", "r") as f:
        for line in f:
            sum_p1 += parse_and_run_instruction(line)

            current_sum, enable_mul = parse_and_run_instruction_with_conditions(line, enable_mul)
            sum_p2 += current_sum

    print("Part-1: ",sum_p1)
    print("Part-2: ",sum_p2)


def parse_and_run_instruction(line: str):
    regex = "mul\((\d+),(\d+)\)"
    regex = re.compile(regex)

    matches = re.findall(regex, line)

    sum = 0
    for match in matches:
        a = int(match[0])
        b = int(match[1])

        sum += a * b

    return sum

# Part: Two

def parse_and_run_instruction_with_conditions(line: str, enable_mul: bool = True):
    """Use regex to parse the statement. Possible tokens are:
    - mul(a, b): Multiply a by b
    - do(): enable multiplication
    - dont(): disable multiplication

    The statement is string containing these tokens as well as many other characters
    that should be ignored.

    The function returns the sum of all multiplications and last state of the multiplication
    flag (True if multiplication is enabled, False otherwise).
    """
    regex = "mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don\'t\(\))"
    regex = re.compile(regex)

    matches = re.finditer(regex, line)

    sum = 0
    do_mul = enable_mul

    for match in matches:

        if match.group() == "do()":
            do_mul = True
        elif match.group() == "don't()":
            do_mul = False

        elif match.group().startswith("mul("):

            a = int(match.group(1))
            b = int(match.group(2))

            if do_mul:
                sum += a * b

    return sum, do_mul



if __name__ == '__main__':
    main()
