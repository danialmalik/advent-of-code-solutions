use std::fs;
use std::iter::repeat;

const IS_TEST: bool = false;

#[derive(Debug, Clone)]
struct DiskBlock {
    starting_position: usize,
    size: usize,
    file_id: Option<usize>,
}

impl DiskBlock {
    fn new(starting_position: usize, size: usize, file_id: Option<usize>) -> Self {
        Self {
            starting_position,
            size,
            file_id,
        }
    }
}

fn main() {
    let script_dir = std::env::current_dir().unwrap();
    let input_path = script_dir.join("input.txt");

    let line: String;

    if IS_TEST {
        line = "2333133121414131402".to_string();
    } else {
        line = fs::read_to_string(input_path).unwrap();
    }



    solve_part_1(&line);
    solve_part_2(&line);
}

fn solve_part_1(line: &str) {
    let mut current_id_number: u32 = 0;
    let mut disk_array: Vec<String> = Vec::new();

    for (i, char) in line.trim().chars().enumerate() {
        let count = char.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            disk_array.extend(repeat(current_id_number.to_string()).take(count));
            current_id_number += 1;
        } else {
            disk_array.extend(repeat(".".to_string()).take(count));
        }
    }

    let mut free_space_index = 0;
    let mut rearrangement_block_index = disk_array.len() - 1;

    while free_space_index < rearrangement_block_index {
        if disk_array[free_space_index] != "." {
            free_space_index += 1;
            continue;
        }

        if disk_array[rearrangement_block_index] == "." {
            rearrangement_block_index -= 1;
            continue;
        }

        disk_array.swap(free_space_index, rearrangement_block_index);
        free_space_index += 1;
        rearrangement_block_index -= 1;
    }

    let checksum: usize = disk_array
        .iter()
        .enumerate()
        .take_while(|(_, file_id)| *file_id != ".")
        .map(|(i, file_id)| i * file_id.parse::<usize>().unwrap())
        .sum();

    println!("Part-1 checksum: {}", checksum);
}

fn solve_part_2(line: &str) {
    let mut current_file_id = 0;
    let mut current_position = 0;
    let mut file_blocks = Vec::new();
    let mut free_blocks = Vec::new();

    for (i, char) in line.trim().chars().enumerate() {
        let size = char.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            file_blocks.push(DiskBlock::new(current_position, size, Some(current_file_id)));
            current_file_id += 1;
        } else {
            free_blocks.push(DiskBlock::new(current_position, size, None));
        }
        current_position += size;
    }

    visualize_line(&file_blocks , &free_blocks);

    for i in (0..file_blocks.len()).rev() {

        for j in 0..free_blocks.len() {

            if free_blocks[j].size >= file_blocks[i].size
                && free_blocks[j].starting_position < file_blocks[i].starting_position
            {
                // Add free block in place of the current file block.
                free_blocks.push(DiskBlock::new(
                    file_blocks[i].starting_position,
                    file_blocks[i].size,
                    None,
                ));

                file_blocks[i].starting_position = free_blocks[j].starting_position;

                if free_blocks[j].size == file_blocks[i].size {
                    free_blocks.remove(j);

                } else {
                    free_blocks[j].starting_position += file_blocks[i].size;
                    free_blocks[j].size -= file_blocks[i].size;
                }

                free_blocks = defrag_free_blocks(free_blocks);

                break;
            }
        }

        visualize_line(&file_blocks , &free_blocks);
    }

    let checksum: usize = file_blocks
        .iter()
        .map(|file_block| {
            (0..file_block.size)
                .map(|idx| {
                    (file_block.starting_position + idx) * file_block.file_id.unwrap()
                })
                .sum::<usize>()
        })
        .sum();

    println!("Part-2 checksum: {}", checksum);
}

fn visualize_line(file_blocks: &[DiskBlock], free_blocks: &[DiskBlock]) {
    if ! IS_TEST {
        return
    }

    let mut all_blocks = file_blocks.to_vec();
    all_blocks.extend_from_slice(free_blocks);
    all_blocks.sort_by_key(|block| block.starting_position);

    for block in all_blocks {
        let char = block.file_id.map_or('.', |id| std::char::from_digit(id as u32, 10).unwrap());
        print!("{}", char.to_string().repeat(block.size));
    }
    println!();
}


fn defrag_free_blocks(free_blocks: Vec<DiskBlock>) -> Vec<DiskBlock> {
    let mut free_blocks = free_blocks;
    free_blocks.sort_by_key(|block| block.starting_position);

    let mut new_free_blocks = Vec::new();

    for i in 0..free_blocks.len() - 1 {
        let current_block = free_blocks[i].clone();
        let next_block = free_blocks[i + 1].clone();

        if new_free_blocks.is_empty() {
            new_free_blocks.push(DiskBlock::new(
                current_block.starting_position,
                current_block.size,
                None,
            ));
            continue;
        }

        if new_free_blocks.last().unwrap().starting_position + new_free_blocks.last().unwrap().size
            == current_block.starting_position
        {
            new_free_blocks.last_mut().unwrap().size += current_block.size;
        } else {
            new_free_blocks.push(DiskBlock::new(
                next_block.starting_position,
                next_block.size,
                None,
            ));
        }
    }

    free_blocks
}

// Python code
// def defrag_free_blocks(free_blocks: list[DiskBlock]) -> list[DiskBlock]:
//     free_blocks = sorted(free_blocks, key=lambda x: x.starting_position)
//     new_free_blocks = []

//     for i in range(len(free_blocks)-1):
//         current_block = free_blocks[i]
//         next_block = free_blocks[i+1]


//         if len(new_free_blocks) == 0:
//             new_free_blocks.append(DiskBlock(
//                 starting_position=current_block.starting_position,
//                 size=current_block.size
//             ))
//             continue

//         if new_free_blocks[-1].starting_position + new_free_blocks[-1].size == current_block.starting_position:
//             new_free_blocks[-1].size += current_block.size
//         else:
//             new_free_blocks.append(DiskBlock(
//                 starting_position=next_block.starting_position,
//                 size=next_block.size
//             ))

//     return free_blocks
