import os
from typing import Union

IS_TEST_INPUT = False


class DiskBlock:
    starting_position: int
    size: int
    file_id: Union[int, None]

    def __init__(self, starting_position, size, file_id=None) -> None:
        self.starting_position = starting_position
        self.size = size
        self.file_id = file_id

    def __str__(self) -> str:
        return "<DiskBlock start={} size={} file_id={}>".format(self.starting_position, self.size, self.file_id)

    def __repr__(self) -> str:
        return self.__str__()


def main():
    script_dir = os.path.dirname(__file__)


    with open(f"{script_dir}/input.txt", "r") as f:
        line = f.readlines()[0]

        if IS_TEST_INPUT:
            line = "2333133121414131402"

        solve_part_1(line)
        solve_part_2(line)


def solve_part_1(line: str):
    current_id_number = 0
    disk_array = []


    for i, char in enumerate(line.strip()):
        if i % 2 == 0:
            disk_array.extend([str(current_id_number)] * int(char))
            current_id_number += 1
        else:
            disk_array.extend(["."] * int(char))

    free_space_index = 0
    re_arrangement_block_index = len(disk_array) - 1

    if IS_TEST_INPUT:
        print("".join(disk_array))

    while free_space_index < re_arrangement_block_index:
        if disk_array[free_space_index] != ".":
            free_space_index += 1
            continue

        if disk_array[re_arrangement_block_index] == ".":
            re_arrangement_block_index -= 1
            continue

        disk_array[free_space_index] = disk_array[re_arrangement_block_index]
        disk_array[re_arrangement_block_index] = "."

        re_arrangement_block_index -= 1
        free_space_index += 1

        if IS_TEST_INPUT:
            print("".join(disk_array))


    checksum = 0
    for i, file_id in enumerate(disk_array):
        if file_id == ".":
            break
        checksum += i * int(file_id)


    print(checksum)


def solve_part_2(line: str):
    current_file_id = 0
    current_position = 0
    file_blocks: list[DiskBlock] = []
    free_blocks: list[DiskBlock] = []

    for i, char in enumerate(line.strip()):
        if i % 2 == 0:
            file_blocks.append(
                DiskBlock(
                    starting_position=current_position,
                    size=int(char),
                    file_id=str(current_file_id)
                )
            )
            current_file_id += 1
            current_position += int(char)

        else:
            free_blocks.append(
                DiskBlock(
                    starting_position=current_position,
                    size=int(char),
                )
            )
            current_position += int(char)

    visualize_line(file_blocks, free_blocks)

    for i in range(len(file_blocks)-1, -1, -1):
        file_block = file_blocks[i]

        for j in range(len(free_blocks)):
            free_block = free_blocks[j]

            if (
                free_block.size >= file_block.size and
                free_block.starting_position < file_block.starting_position
            ):
                free_blocks.append(DiskBlock(
                    starting_position=file_block.starting_position,
                    size=file_block.size,
                ))

                file_block.starting_position = free_block.starting_position

                if free_block.size == file_block.size:
                    free_blocks.pop(j)

                else:
                    free_block.size -= file_block.size
                    free_block.starting_position += file_block.size

                free_blocks = defrag_free_blocks(free_blocks)

                break

        visualize_line(file_blocks, free_blocks)


    checksum = 0

    for file_block in file_blocks:
        checksum += sum([
            (file_block.starting_position + idx) * int(file_block.file_id) for idx in range(file_block.size)
        ])

    print(checksum)


def defrag_free_blocks(free_blocks: list[DiskBlock]) -> list[DiskBlock]:
    free_blocks = sorted(free_blocks, key=lambda x: x.starting_position)
    new_free_blocks = []

    for i in range(len(free_blocks)-1):
        current_block = free_blocks[i]
        next_block = free_blocks[i+1]


        if len(new_free_blocks) == 0:
            new_free_blocks.append(DiskBlock(
                starting_position=current_block.starting_position,
                size=current_block.size
            ))
            continue

        if new_free_blocks[-1].starting_position + new_free_blocks[-1].size == current_block.starting_position:
            new_free_blocks[-1].size += current_block.size
        else:
            new_free_blocks.append(DiskBlock(
                starting_position=next_block.starting_position,
                size=next_block.size
            ))

    return free_blocks


def visualize_line(file_blocks: list[DiskBlock], free_blocks: list[DiskBlock]):
    if not IS_TEST_INPUT:
        return

    all_blocks = [
        *file_blocks,
        *free_blocks
    ]

    all_blocks = sorted(all_blocks, key=lambda x: x.starting_position)

    for block in all_blocks:
        char = block.file_id if block.file_id is not None else "."
        print(char * block.size, end="")
    print("")

if __name__ == "__main__":
    main()
