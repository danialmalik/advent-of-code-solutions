use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Define the input file path
    let input_path = "input.txt";

    // Open the file in read-only mode
    let file = File::open(&input_path)?;

    // Create a buffered reader for efficient reading
    let reader = io::BufReader::new(file);

    // Initialize two vectors to store the numbers
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap the line
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // Parse the numbers and add them to the respective lists
        if let (Some(first), Some(second)) = (numbers.get(0), numbers.get(1)) {
            if let (Ok(first_num), Ok(second_num)) = (first.parse::<i32>(), second.parse::<i32>()) {
                first_list.push(first_num);
                second_list.push(second_num);
            }
        }
    }

    // Sort the two lists
    first_list.sort();
    second_list.sort();

    // Find and sum difference of the two lists index by index
    let mut sum = 0;
    for i in 0..first_list.len() {
        sum += (first_list[i] - second_list[i]).abs();
    }

    // Print the sum
    println!("{}", sum);

    // PART TWO

    // Make an occurence map for the second list
    let mut occurence_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for num in second_list {
        let count = occurence_map.entry(num).or_insert(0);
        *count += 1;
    }

    // Find similarity score
    let mut score = 0;
    for num in first_list {
        if let Some(count) = occurence_map.get_mut(&num) {
            if *count > 0 {
                score += num * *count;
            }
        }
    }

    // Print the score
    println!("{}", score);

    Ok(())
}
