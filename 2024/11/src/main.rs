use std::fs;
use std::collections::HashMap;


fn main() {
    let script_dir = std::env::current_dir().unwrap();
    let input_path = script_dir.join("input.txt");

    let content = fs::read_to_string(input_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let line = lines[0];



    // Part-1
    let result = find_stones_count_after_n_blinks(line, 25);
    println!("Part-1: {}", result);

    // Part-2
    let result = find_stones_count_after_n_blinks(line, 75);
    println!("Part-2: {}", result);

}


fn find_stones_count_after_n_blinks(line: &str, steps: usize) -> usize {
    let stones: Vec<&str> = line.split(" ").collect();
    let mut cache_store = HashMap::<(String, usize), usize>::new();

    return stones.iter().map(|x| find_stones_count_after_n_blinks_recursive(*x, steps, &mut cache_store)).sum();
}


fn find_stones_count_after_n_blinks_recursive(stone: &str, steps: usize, cache_store: &mut HashMap<(String, usize), usize>) -> usize {
    if let Some(&value) = cache_store.get(&(stone.to_string(), steps)) {
        return value;
    }

    let result: usize;
    let cache_key: (String, usize) = (stone.to_string(), steps);

    if steps == 0 {

        return 1;

    } else if stone == "0" {

        result = find_stones_count_after_n_blinks_recursive("1", steps-1, cache_store);

    } else if stone.len() % 2 == 0 {

        let left_half = &stone[0..stone.len()/2].parse::<usize>().unwrap().to_string();
        let right_half = &stone[stone.len()/2..stone.len()].parse::<usize>().unwrap().to_string();

        result = find_stones_count_after_n_blinks_recursive(
            left_half, steps-1, cache_store
        ) + find_stones_count_after_n_blinks_recursive(
            right_half, steps-1, cache_store
        );

    } else {
        // Mul stone value with 2024
        let product = stone.parse::<usize>().unwrap() * 2024;

        result = find_stones_count_after_n_blinks_recursive(
            &product.to_string(), steps-1, cache_store
        );
    }

    cache_store.insert(cache_key, result);
    return result;

}
