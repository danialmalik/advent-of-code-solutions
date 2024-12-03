import os
import re


def main():
    script_dir = os.path.dirname(__file__)
    with open(f"{script_dir}/input.txt") as f:
        lines = f.readlines()

    lines = [line.strip() for line in lines]

    sum = 0
    for line in lines:
        sum += parse_and_run_instruction(line)

    print(sum)

def parse_and_run_instruction(line: str):
    regex = "mul\((\d+),(\d+)\)"

    matches = re.findall(regex, line)

    sum = 0
    for match in matches:
        a = int(match[0])
        b = int(match[1])

        sum += a * b

    return sum



if __name__ == '__main__':
    main()
