from collections import defaultdict
import os



def main():
    script_dir = os.path.dirname(__file__)

    grid = defaultdict(list)
    max_x, max_y = 0, 0

    with open(f"{script_dir}/input.txt", "r") as f:
        lines = f.readlines()
        max_y = len(lines)
        max_x = len(lines[0].strip())

        for y, line in enumerate(lines):
            for x, char in enumerate(line.strip()):
                if char != ".":
                    existing = grid[char]
                    if existing is None:
                        grid[char] = []
                    grid[char].append((x, y))

    antinodes = set()

    # Part-2
    all_antinodes = set()

    for antenna_char in grid:
        antennas = grid[antenna_char]
        for i in range(len(antennas)-1):
            for j in range(i+1, len(antennas)):
                antenna_a = antennas[i]
                antenna_b = antennas[j]

                possible_antinodes = generate_antinodes(antenna_a, antenna_b)
                for antinode in possible_antinodes:
                    if is_valid(antinode, max_x, max_y):
                        antinodes.add(antinode)

                # Part-2
                all_antinodes |= generate_all_antinodes(antenna_a, antenna_b, max_x, max_y)

    print(len(antinodes))
    print(len(all_antinodes))


def is_valid(position: tuple[int, int], max_x: int, max_y: int):
    return 0 <= position[0] < max_x and 0 <= position[1] < max_y


def generate_antinodes(p1: tuple[int, int], p2: tuple[int, int]) -> tuple[tuple[int, int], tuple[int, int]]:

    dx = p2[0] - p1[0]
    dy = p2[1] - p1[1]

    antinode_a = p1[0] - dx, p1[1] - dy
    antinode_b = p2[0] + dx, p2[1] + dy

    return antinode_a, antinode_b

# Part-2:
def generate_all_antinodes(
    antenna_a: tuple[int, int],
    antenna_b: tuple[int, int],
    max_x: int,
    max_y: int
) -> set[tuple[int, int]]:
    antinodes = set()

    dx = antenna_b[0] - antenna_a[0]
    dy = antenna_b[1] - antenna_a[1]

    current = antenna_a
    while is_valid(current, max_x, max_y):
        antinodes.add(current)
        current = current[0] + dx, current[1] + dy

    current = antenna_b
    while is_valid(current, max_x, max_y):
        antinodes.add(current)
        current = current[0] - dx, current[1] - dy

    return antinodes


if __name__ == "__main__":
    main()
