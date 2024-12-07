import itertools
import os

test_input = """190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"""


def main():
    script_dir = os.path.dirname(__file__)

    final_result = 0

    with open(f"{script_dir}/input.txt", "r") as f:

        # for line in test_input.splitlines():
        for line in f:

            result, operands_str = line.split(': ')
            result = int(result)
            operands = list(map(int, operands_str.split(' ')))
            operators = ['+', '*']

            for op_permutation in itertools.product(operators, repeat=len(operands) - 1):

                computed_result = operands[0]

                for operator, operand in zip(op_permutation, operands[1:]):
                    if operator == '+':
                        computed_result += operand
                    elif operator == '*':
                        computed_result *= operand

                if computed_result == result:
                    final_result += result
                    break

    print("Part-1: ", final_result)


if __name__ == "__main__":
    main()
