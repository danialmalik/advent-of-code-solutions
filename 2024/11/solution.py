from collections import defaultdict
from functools import cache
import os

TEST_INPUT = "0 1 10 99 999"

def main():
    script_dir = os.path.dirname(__file__)


    with open(f"{script_dir}/input.txt", "r") as f:
        lines = f.readlines()

    line = lines[0].strip()

    print("TEST Input 1: ")
    print(find_stones_count_after_blinks(TEST_INPUT, n=1))

    print("Input: ")
    print(find_stones_count_after_blinks(line, n=25))

    print("Part-2")
    print(find_stones_count_after_blinks(line, n=75))


def find_stones_count_after_blinks(input_str: str, n: int):
    stones = [x for x in input_str.strip().split(" ")]

    return sum([
        find_stones_count_after_blinks_recursive(stone, n) for stone in stones
    ])

@cache
def find_stones_count_after_blinks_recursive(stone: str, n: int):
    if n == 0:
        return 1

    if stone == "0":
        return find_stones_count_after_blinks_recursive("1", n-1)

    if len(stone) % 2 == 0:
        return find_stones_count_after_blinks_recursive(
            str(int(stone[:len(stone)//2])), n-1
        ) + find_stones_count_after_blinks_recursive(
            str(int(stone[len(stone)//2:])), n-1
        )

    return find_stones_count_after_blinks_recursive(str(int(stone) * 2024), n-1)


if __name__ == "__main__":
    main()
