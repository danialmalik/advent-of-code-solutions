from collections import defaultdict
from functools import cmp_to_key
import os
from typing import Callable, DefaultDict


def main():
    script_dir = os.path.dirname(__file__)

    grid: DefaultDict = defaultdict(lambda: None)

    starting_position: tuple[int, int] = None

    max_row, max_col = 0, 0

    with open(f"{script_dir}/input.txt", "r") as f:
        lines = f.readlines()
        max_row = len(lines)
        max_col = len(lines[0])

        for row_num, line in enumerate(lines):
            for col_num, char in enumerate(line.strip()):
                if char == "#":
                    grid[(row_num, col_num)] = '#'
                elif char == '^':
                    starting_position = (row_num, col_num)


    starting_direction = (-1, 0) # Up
    visited, _ = map_patrol_route(grid, starting_position, starting_direction, max_row, max_col)
    possible_obstacles = find_possible_obstacles_for_loop(visited, grid, starting_position, starting_direction, max_row, max_col)

    print("Part-1: ", len({k for k, v in visited}))
    print("Part-2: ", len(possible_obstacles))


def map_patrol_route(
        grid: DefaultDict,
        starting_position: tuple[int, int],
        starting_direction: tuple[int, int],
        max_row: int,
        max_col: int
) -> tuple[list[tuple[int, int]], bool]:

    visited = list()
    current_position = starting_position
    current_direction = starting_direction

    while not is_out_of_map(current_position, max_row, max_col):
        visited.append((current_position, current_direction))

        current_direction = get_next_direction(current_position, current_direction, grid)
        current_position = (current_position[0] + current_direction[0], current_position[1] + current_direction[1])

        if (current_position, current_direction) in visited:
            return visited, True

    return visited, False


def find_possible_obstacles_for_loop(
    visited: list[tuple[int, int]],
    grid: DefaultDict,
    starting_position: tuple[int, int],
    starting_direction: tuple[int, int],
    max_row: int,
    max_col: int
) -> set[tuple[int, int]]:
    possible_obstacles = set()

    for position, direction in visited:

        grid[position] = "#"

        _, is_loop = map_patrol_route(
            grid=grid,
            starting_position=starting_position,
            starting_direction=starting_direction,
            max_row=max_row,
            max_col=max_col
        )

        if is_loop:
            possible_obstacles.add(position)

        del grid[position]

    return possible_obstacles



def is_out_of_map(position: tuple[int, int], max_row: int, max_col: int) -> bool:
    return position[0] < 0 or position[0] >= max_row or position[1] < 0 or position[1] >= max_col


def get_next_direction(
        current_position: tuple[int, int],
        current_direction: tuple[int, int],
        grid: DefaultDict
    ) -> tuple[int, int]:
    """Rotate current direction 90 degrees right/clockwise"""
    directions = [
        (-1, 0), # Up
        (0, 1),  # Right
        (1, 0),  # Down
        (0, -1)  # Left
    ]

    idx_current = directions.index(current_direction)

    if grid[(current_position[0] + current_direction[0], current_position[1] + current_direction[1])] != "#":
        return current_direction

    for i in range(4):
        next_direction = directions[(idx_current + i) % 4]
        if grid[(current_position[0] + next_direction[0], current_position[1] + next_direction[1])] != "#":
            return next_direction

    raise ValueError("No valid direction found")

if __name__ == "__main__":
    main()
