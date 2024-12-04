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
                total_xmas_occurrences += find_xmas_occurrences(&grid, i, j);
            }
            // Part-2
            else if *cell == 'A' {
                total_cross_mas_occurrences += find_x_mas_occurrences(&grid, i, j) as u32;
            }
        }
    }

    println!("Part-1: Total Xmas occurrences: {}", total_xmas_occurrences);
    println!("Part-2: Total Cross-Mas occurrences: {}", total_cross_mas_occurrences);

    Ok(())
}

fn find_xmas_occurrences(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut xmas_occurrences = 0;
    const TARGET: &str = "XMAS";

    let directions: [[i8; 2]; 8] = [
        [1,0], // right
        [1,1], // right-down
        [0,1], // down
        [-1,1], // left-down
        [-1,0], // left
        [-1,-1], // left-up
        [0,-1], // up
        [1,-1], // right-up
    ];

   for direction in directions {
        let mut x: i32 = i as i32;
        let mut y: i32 = j as i32;
        let mut k: u8 = 1;
        let mut found: bool = true;

        while (k as usize) < TARGET.len() {
            x += direction[0] as i32;
            y += direction[1] as i32;

            if !is_valid_pos(grid, x, y) || grid[x as usize][y as usize] != TARGET.chars().nth(k as usize).unwrap() {
                found = false;
                break;
            }

            k += 1;
        }

        if found {
            xmas_occurrences += 1;
        }
    }

    xmas_occurrences
}

fn find_x_mas_occurrences(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u8 {
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

    let reading_direction: [[i8; 2]; 5] = [
        [-1, 1], // left-up
        [1, 1], // right-up
        [0, 0], // Center
        [-1, -1], // left-down
        [1, -1], // right-down
    ];

    let mut candidate: String = String::new();

    for direction in reading_direction {
        let x: i32 = i as i32 + direction[0] as i32;
        let y: i32 = j as i32 + direction[1] as i32;
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

fn is_valid_pos(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    i >= 0 && i < (grid.len() as i32) && j >= 0 && j < (grid[i as usize].len() as i32)
}
