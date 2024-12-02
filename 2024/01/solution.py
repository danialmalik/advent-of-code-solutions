import os


def main():
    script_dir = os.path.dirname(__file__)
    with open(f"{script_dir}/input.txt") as f:
        lines = f.readlines()

    lines = [line.strip() for line in lines]

    left_group, right_group = [], []

    for line in lines:
        left, right = line.split("   ")
        left_group.append(int(left))
        right_group.append(int(right))

    left_group.sort()
    right_group.sort()

    total_diff = 0

    for i in range(len(left_group)):
        total_diff += abs(left_group[i] - right_group[i])

    print(total_diff)


if __name__ == '__main__':
    main()
