use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let input_path = "input.txt";
    let file = File::open(&input_path)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut total_xmas_occurrences: u32 = 0;
    let mut total_cross_mas_occurrences: u32 = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {

            // Part-1:
            if *cell == 'X' {
                total_xmas_occurrences += find_xmas_occurrences(&grid, i.try_into().unwrap(), j.try_into().unwrap());
            }
            // Part-2
            else if *cell == 'A' {
                total_cross_mas_occurrences += find_x_mas_occurrences(&grid, i.try_into().unwrap(), j.try_into().unwrap()) as u32;
            }
        }
    }

    println!("Part-1: Total Xmas occurrences: {}", total_xmas_occurrences);
    println!("Part-2: Total Cross-Mas occurrences: {}", total_cross_mas_occurrences);

    Ok(())
}

fn find_xmas_occurrences(grid: &[Vec<char>], i: isize, j: isize) -> u32 {
    const TARGET: &[char] = &['X', 'M', 'A', 'S'];
    let directions: &[(isize, isize)] = &[
        (1, 0), (1, 1), (0, 1), (-1, 1),
        (-1, 0), (-1, -1), (0, -1), (1, -1),
    ];

    directions.iter().filter(|&&(dx, dy)| {
        (0..TARGET.len()).all(|k| {
            let x = i + k as isize * dx;
            let y = j + k as isize * dy;
            is_valid_pos(grid, x, y) && grid[x as usize][y as usize] == TARGET[k]
        })
    }).count() as u32
}

fn find_x_mas_occurrences(grid: &Vec<Vec<char>>, i: isize, j: isize) -> u8 {
    // Find count of X-MAS in the grid
    // X-MAS means two MAS in cross. like

    // M . M
    // . A .
    // S . S

    // Possible positions are:

    // M . M
    // . A .
    // S . S

    // S . S
    // . A .
    // M . M

    // M . S
    // . A .
    // M . S

    // S . M
    // . A .
    // S . M

    let possible_combinations: [&str; 4] = [
        "MMASS",
        "SSAMM",
        "MSAMS",
        "SMASM",
    ];

    let reading_direction: [[isize; 2]; 5] = [
        [-1, 1], // left-up
        [1, 1], // right-up
        [0, 0], // Center
        [-1, -1], // left-down
        [1, -1], // right-down
    ];

    let mut candidate: String = String::new();

    for direction in reading_direction {
        let x = i + direction[0];
        let y = j + direction[1];
        if is_valid_pos(grid, x, y) {
            candidate.push(grid[(x) as usize][(y) as usize]);
        }
    }

    for combination in possible_combinations {
        if candidate == combination {
            return 1;
        }
    }
    0
}

fn is_valid_pos(grid: &[Vec<char>], i: isize, j: isize) -> bool {
    i >= 0 && i < grid.len() as isize && j >= 0 && j < grid[i as usize].len() as isize
}
