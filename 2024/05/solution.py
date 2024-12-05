from collections import defaultdict
from functools import cmp_to_key
import os
from typing import Callable, DefaultDict


def main():
    script_dir = os.path.dirname(__file__)

    rules : list[tuple[int, int]] = []
    updates: list[list[str]] = []

    with open(f"{script_dir}/input.txt", "r") as f:
        for line in f:
            if line == "\n":
                break
            rules.append(tuple([int(x) for x in line.strip().split("|")]))
        for line in f:
            updates.append([int(x) for x  in line.strip().split(",")])

    requirement_map: DefaultDict[int, set[int]] = defaultdict()

    for rule in rules:
        if rule[1] not in requirement_map:
            requirement_map[rule[1]] = set()
        requirement_map[rule[1]].add(rule[0])

    middle_page_sum = 0
    middle_page_sum_p2 = 0

    compare_fn = make_comparator(requirement_map)
    for update in updates:
        visited = set()

        for page in update:
            if requirement_map[page] is not None and len(requirement_map[page].intersection(visited)) != len(visited):
                sorted_update = sorted(update, key=cmp_to_key(compare_fn))
                middle_page_sum_p2 += sorted_update[len(sorted_update)//2]
                break
            visited.add(page)
        else:
            middle_page_sum += update[len(update) // 2]

    print(middle_page_sum)
    print(middle_page_sum_p2)


def make_comparator(requirement_map: DefaultDict[int, set[int]]) -> Callable:

    def _comparator(a: int, b: int) -> int:
        if a in requirement_map[b]:
            return -1
        if b in requirement_map[a]:
            return 1
        return 0

    return _comparator



if __name__ == "__main__":
    main()
