from collections import defaultdict
import os



def main():
    script_dir = os.path.dirname(__file__)

    disk_array = []

    with open(f"{script_dir}/input.txt", "r") as f:
        line = f.readlines()[0]

        current_id_number = 0

        for i, char in enumerate(line.strip()):
            if i % 2 == 0:
                disk_array.extend([str(current_id_number)] * int(char))
                current_id_number += 1
            else:
                disk_array.extend(["."] * int(char))

    # print(disk_array)

    # ReArrange the disk

    free_space_index = 0
    re_arrangement_block_index = len(disk_array) - 1

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

    checksum = 0
    for i, file_id in enumerate(disk_array):
        if file_id == ".":
            break
        checksum += i * int(file_id)


    print(checksum)

if __name__ == "__main__":
    main()
