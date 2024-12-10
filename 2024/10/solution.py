from collections import defaultdict
import os
from typing import DefaultDict

TEST_INPUT = """...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
"""

TEST_INPUT_2 = """89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"""

TEST_INPUT_P2 = """.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
"""

TEST_INPUT_P2_2 = """..90..9
...1.98
...2..7
6543456
765.987
876....
987....
"""


def main():
    script_dir = os.path.dirname(__file__)


    with open(f"{script_dir}/input.txt", "r") as f:
        lines = f.readlines()

    print("TEST Input 1: ")
    find_trailhead_score(TEST_INPUT.splitlines())

    print("TEST Input 2: ")
    find_trailhead_score(TEST_INPUT_2.splitlines())

    print("Input: ")
    find_trailhead_score(lines)

    ## Part 2
    print("===== Part-2")
    print("TEST Input 1: ")
    find_trailhead_score(TEST_INPUT_P2.splitlines(), distinct_peaks=False)

    print("TEST Input 2: ")
    find_trailhead_score(TEST_INPUT_P2_2.splitlines(), distinct_peaks=False)

    print("Input: ")
    find_trailhead_score(lines, distinct_peaks=False)


def find_trailhead_score(lines: list[str], distinct_peaks: bool = True):
    grid = defaultdict(lambda: None)
    max_x = len(lines[0])
    max_y = len(lines)

    for i, line in enumerate(lines):

        for j, char in enumerate(line.strip()):
            if char != ".":
                grid[(j, i)] = int(char)


    score = 0
    for entries in grid.items():
        x, y = entries[0]
        height = entries[1]

        if height == 0:
            peaks = []
            find_trailhead_score_recursive(grid, x, y, max_x, max_y, peaks)
            if distinct_peaks:
                score += len(set(peaks))
            else:
                score += len(peaks)

    print(score)

def find_trailhead_score_recursive(
    grid: DefaultDict,
    x: int,
    y: int,
    max_x: int,
    max_y: int,
    all_peaks: list[tuple[int, int]]
):
    if not is_valid(x, y, max_x, max_y):
        return

    current_height = grid[(x, y)]

    if current_height == 9:
        all_peaks.append((x, y))
        return

    directions = [
        (0, -1), # up
        (0, 1), # down
        (-1, 0), # left
        (1, 0), # right
    ]

    for dx, dy in directions:
        new_x = x + dx
        new_y = y + dy

        if (
            is_valid(new_x, new_y, max_x, max_y) and
            (new_x, new_y) in grid and
            grid[(new_x, new_y)] == current_height + 1
        ):
            find_trailhead_score_recursive(grid, new_x, new_y, max_x, max_y, all_peaks)

    return

def is_valid(x, y, max_x, max_y):
    return x >= 0 and x < max_x and y >= 0 and y < max_y



if __name__ == "__main__":
    main()
