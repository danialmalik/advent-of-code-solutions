use std::collections::HashMap;
use std::fs;

fn main() {
    let script_dir = std::env::current_dir().unwrap();
    let input_path = script_dir.join("input.txt");
    let lines = fs::read_to_string(input_path).unwrap();
    let input_lines: Vec<&str> = lines.lines().collect();

    println!("Input:");
    find_trailhead_score(input_lines.clone(), true);

    // Part 2
    println!("===== Part-2 =====");
    println!("Input:");
    find_trailhead_score(input_lines, false);
}

fn find_trailhead_score(lines: Vec<&str>, distinct_peaks: bool) {
    let mut grid: HashMap<(usize, usize), u32> = HashMap::new();
    let max_x = lines[0].len();
    let max_y = lines.len();

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char != '.' {
                grid.insert((j, i), char.to_digit(10).unwrap());
            }
        }
    }

    let mut score = 0;

    for (&(x, y), &height) in &grid {
        if height == 0 {
            let mut peaks = Vec::new();
            find_trailhead_score_recursive(&grid, x, y, max_x, max_y, &mut peaks);

            if distinct_peaks {
                let unique_peaks: std::collections::HashSet<_> = peaks.into_iter().collect();
                score += unique_peaks.len();
            } else {
                score += peaks.len();
            }
        }
    }

    println!("{}", score);
}

fn find_trailhead_score_recursive(
    grid: &HashMap<(usize, usize), u32>,
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    all_peaks: &mut Vec<(usize, usize)>,
) {
    if !is_valid(x, y, max_x, max_y) {
        return;
    }

    let current_height = grid.get(&(x, y)).cloned().unwrap_or(0);

    if current_height == 9 {
        all_peaks.push((x, y));
        return;
    }

    let directions = [
        (0, -1),  // up
        (0, 1),   // down
        (-1, 0),  // left
        (1, 0),   // right
    ];

    for (dx, dy) in directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if is_valid(new_x, new_y, max_x, max_y) {
                if let Some(&neighbor_height) = grid.get(&(new_x, new_y)) {
                    if neighbor_height == current_height + 1 {
                        find_trailhead_score_recursive(grid, new_x, new_y, max_x, max_y, all_peaks);
                    }
                }
            }
        }
    }
}

fn is_valid(x: usize, y: usize, max_x: usize, max_y: usize) -> bool {
    x < max_x && y < max_y
}
