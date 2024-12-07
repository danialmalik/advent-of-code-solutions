use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

type Position = (i32, i32);
type Direction = (i32, i32);
type Grid = HashMap<Position, char>;

fn main() -> io::Result<()> {
    let script_dir = std::env::current_dir()?;
    let input_path = script_dir.join("input.txt");

    let content = fs::read_to_string(input_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let max_row = lines.len() as i32;
    let max_col = lines[0].len() as i32;

    let mut grid: Grid = HashMap::new();
    let mut starting_position: Option<Position> = None;

    for (row_num, line) in lines.iter().enumerate() {
        for (col_num, char) in line.chars().enumerate() {
            let position = (row_num as i32, col_num as i32);
            match char {
                '#' => {
                    grid.insert(position, '#');
                }
                '^' => {
                    starting_position = Some(position);
                }
                _ => {}
            }
        }
    }

    let starting_position = starting_position.expect("Starting position not found");
    let starting_direction: Direction = (-1, 0); // Up

    let (visited, _) = map_patrol_route(
        &grid, starting_position, starting_direction, max_row, max_col
    );
    let possible_obstacles = find_possible_obstacles_for_loop(
        &visited,
        &mut grid,
        starting_position,
        starting_direction,
        max_row,
        max_col
    );

    println!("Part-1: {}", visited.iter().map(|(pos, _)| *pos).collect::<HashSet<_>>().len());
    println!("Part-2: {}", possible_obstacles);

    Ok(())
}

fn map_patrol_route(
    grid: &Grid,
    starting_position: Position,
    starting_direction: Direction,
    max_row: i32,
    max_col: i32,
) -> (HashSet<(Position, Direction)>, bool) {
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let mut current_position = starting_position;
    let mut current_direction = starting_direction;

    while !is_out_of_map(current_position, max_row, max_col) {
        visited.insert((current_position, current_direction));

        current_direction = get_next_direction(current_position, current_direction, grid);
        current_position = (
            current_position.0 + current_direction.0,
            current_position.1 + current_direction.1,
        );

        if visited.contains(&(current_position, current_direction)) {
            return (visited, true);
        }
    }

    (visited, false)
}

fn find_possible_obstacles_for_loop(
    visited: &HashSet<(Position, Direction)>,
    grid: &mut Grid,
    starting_position: Position,
    starting_direction: Direction,
    max_row: i32,
    max_col: i32,
) -> u32 {
    let mut possible_obstacles: u32 = 0;

    for &(position, _direction) in visited {
        grid.insert(position, '#');

        let (_, is_loop) = map_patrol_route(grid, starting_position, starting_direction, max_row, max_col);

        if is_loop {
            possible_obstacles += 1;
        }

        grid.remove(&position);
    }

    possible_obstacles
}

fn is_out_of_map(position: Position, max_row: i32, max_col: i32) -> bool {
    position.0 < 0 || position.0 >= max_row || position.1 < 0 || position.1 >= max_col
}

fn get_next_direction(
    current_position: Position,
    current_direction: Direction,
    grid: &Grid,
) -> Direction {
    let directions = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    let idx_current = directions.iter().position(|&d| d == current_direction).unwrap();

    if grid
        .get(&(current_position.0 + current_direction.0, current_position.1 + current_direction.1))
        != Some(&'#')
    {
        return current_direction;
    }

    for i in 0..4 {
        let next_direction = directions[(idx_current + i) % 4];
        if grid
            .get(&(current_position.0 + next_direction.0, current_position.1 + next_direction.1))
            != Some(&'#')
        {
            return next_direction;
        }
    }

    panic!("No valid direction found");
}
