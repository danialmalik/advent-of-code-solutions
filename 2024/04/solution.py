import os
import re


def main():
    script_dir = os.path.dirname(__file__)

    sum_p1 = 0
    sum_p2 = 0

    enable_mul = True

    grid = [[]]

    with open(f"{script_dir}/input.txt", "r") as f:
        for line in f:
            grid.append([
                c for c in line.strip()
            ])


    # Find count of XMAS in the grid (part 1)
    xmas_occurrences = 0
    # Count of X-MAS (Part 2)
    x_mas_occurrences = 0
    for i in range(0, len(grid)):
        for j in range(0, len(grid[i])):
            if grid[i][j] == 'X':
                xmas_occurrences += find_xmas(grid, i, j)

            if grid[i][j] == 'A':
                x_mas_occurrences += find_x_mas(grid, i, j)

    print(f"Part 1: {xmas_occurrences}")
    print(f"Part 2: {x_mas_occurrences}")

def find_xmas(grid: list[list[str]], i: int, j: int) -> int:
    # Part 1
    count = 0
    TARGET = "XMAS"

    directions = [
        (1,0), # right
        (1,1), # right-down
        (0,1), # down
        (-1,1), # left-down
        (-1,0), # left
        (-1,-1), # left-up
        (0,-1), # up
        (1,-1), # right-up
    ]

    for direction in directions:
        x = i
        y = j
        for c in TARGET[1:]:
            x += direction[0]
            y += direction[1]
            if not is_valid_pos(grid, x, y) or grid[x][y] != c:
                break
        else:
            count += 1

    return count

def find_x_mas(grid: list[list[str]], i: int, j: int) -> int:
    """Find count of X-MAS in the grid
    X-MAS means two MAS in cross. like

    M . M
    . A .
    S . S

    Possible positions are:

    M . M
    . A .
    S . S

    S . S
    . A .
    M . M

    M . S
    . A .
    M . S

    S . M
    . A .
    S . M
    """
    possible_combinations = [
        "MMASS",
        "SSAMM",
        "MSAMS",
        "SMASM",
    ]

    reading_direction = (
        (-1, 1), # left-up
        (1, 1), # right-up
        (0, 0), # Center
        (-1, -1), # left-down
        (1, -1), # right-down
    )

    candidate = "".join([
        grid[i + direction[0]][j + direction[1]]
        for direction in reading_direction
        if is_valid_pos(grid, i + direction[0], j + direction[1])
    ])

    if candidate in possible_combinations:
        return 1
    return 0

def is_valid_pos(grid, x, y):
    return 0 <= x < len(grid) and 0 <= y < len(grid[x])

if __name__ == '__main__':
    main()
