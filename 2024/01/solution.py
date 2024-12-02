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

    # Part two
    # Find similarity score

    # Make occurrences dict for right group
    right_occurrences = {}

    for num in right_group:
        if num in right_occurrences:
            right_occurrences[num] += 1
        else:
            right_occurrences[num] = 1

    similarity_score = 0

    for num in left_group:
        if num in right_occurrences:
            similarity_score += num * right_occurrences[num]

    print(similarity_score)


if __name__ == '__main__':
    main()
