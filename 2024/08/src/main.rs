use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let script_dir = std::env::current_dir().unwrap();
    let input_path = script_dir.join("input.txt");

    let content = fs::read_to_string(input_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let max_y = lines.len();
    let max_x = lines[0].trim().len();

    let mut grid: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char != '.' {
                grid.entry(char).or_insert_with(Vec::new).push((x as u32, y as u32));
            }
        }
    }

    let mut antinodes: HashSet<(u32, u32)> = HashSet::new();
    let mut all_antinodes: HashSet<(u32, u32)> = HashSet::new();

    for antennas in grid.values() {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let antenna_a = antennas[i];
                let antenna_b = antennas[j];

                let possible_antinodes = generate_antinodes(antenna_a, antenna_b);
                for antinode in possible_antinodes {
                    if is_valid(antinode, max_x as u32, max_y as u32) {
                        antinodes.insert(antinode);
                    }
                }

                // Part-2
                let additional_antinodes = generate_all_antinodes(antenna_a, antenna_b, max_x as u32, max_y as u32);
                all_antinodes.extend(additional_antinodes);
            }
        }
    }

    println!("{}", antinodes.len());
    println!("{}", all_antinodes.len());
}

fn is_valid(position: (u32, u32), max_x: u32, max_y: u32) -> bool {
    position.0 >= 0 && position.0 < max_x && position.1 >= 0 && position.1 < max_y
}

fn generate_antinodes(p1: (u32, u32), p2: (u32, u32)) -> [(u32, u32); 2] {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    let antinode_a = (p1.0 - dx, p1.1 - dy);
    let antinode_b = (p2.0 + dx, p2.1 + dy);

    [antinode_a, antinode_b]
}

fn generate_all_antinodes(
    antenna_a: (u32, u32),
    antenna_b: (u32, u32),
    max_x: u32,
    max_y: u32
) -> HashSet<(u32, u32)> {
    let mut antinodes = HashSet::new();

    let dx = antenna_b.0 - antenna_a.0;
    let dy = antenna_b.1 - antenna_a.1;

    let mut current = antenna_a;
    while is_valid(current, max_x, max_y) {
        antinodes.insert(current);
        current = (current.0 + dx, current.1 + dy);
    }

    current = antenna_b;
    while is_valid(current, max_x, max_y) {
        antinodes.insert(current);
        current = (current.0 - dx, current.1 - dy);
    }

    antinodes
}
