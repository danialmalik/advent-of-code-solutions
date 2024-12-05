use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let input_path = "input.txt";
    let file = File::open(&input_path)?;
    let reader = io::BufReader::new(file);

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut is_parsing_rules = true;
    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            is_parsing_rules = false;
            continue;
        }

        if is_parsing_rules == true {
            let rule: Vec<u32> = line.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
            rules.push((rule[0], rule[1]));
        } else {
            updates.push(line.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }

    let mut requirements_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for rule in rules {
        let (before, after) = rule;
        let entry = requirements_map.entry(after).or_insert(HashSet::new());
        entry.insert(before);
    }

    let mut middles_sum: u32 = 0;
    let mut middle_sum_p2: u32 = 0;

    for update in &updates {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut is_valid: bool = true;

        for page in update {
            if requirements_map.contains_key(&page) && requirements_map[&page].intersection(&visited).count() != visited.len() {
                is_valid = false;

                let mut update_copy = update.clone();
                update_copy.sort_by(|a, b| {
                    if requirements_map[a].contains(b) {
                        return std::cmp::Ordering::Less;
                    } else if requirements_map[b].contains(a) {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Equal;
                    }
                });

                middle_sum_p2 += update_copy[update_copy.len() / 2];

                break;
            }
            visited.insert(*page);
        }

        if is_valid {
            let middle_index = update.len() / 2;
            middles_sum += update[middle_index];

        }
    }

    println!("Part 1: {}", middles_sum);
    println!("Part 2: {}", middle_sum_p2);


    Ok(())
}
