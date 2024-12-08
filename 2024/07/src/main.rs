use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let script_dir = std::env::current_dir()?;
    let input_path = script_dir.join("input.txt");

    let content = fs::read_to_string(input_path)?;
    let lines: Vec<&str> = content.lines().collect();

    println!("Part-1: {}", find_calibration_results(&lines, false));
    println!("Part-2: {}", find_calibration_results(&lines, true));

    Ok(())
}

fn find_calibration_results(lines: &[&str], include_pipe: bool) -> usize {
    let mut final_result = 0;

    let mut operators = vec!["+", "*"];
        if include_pipe {
            operators.push("||");
        }

    let all_operators = operators.clone();

    for line in lines {
        let line = *line;
        let parts: Vec<&str> = line.split(':').collect();
        let result: usize = parts[0].trim().parse().unwrap();

        let operands: Vec<usize> = parts[1]
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();

        // Make permutations of operators with repetition using repeat_n
        let permutations = cartesian_product(&all_operators, operands.len() - 1);

        for perm in permutations {
            let mut computed_result = operands[0] as usize;

            for (operator, operand) in perm.iter().zip(operands.iter().skip(1)) {
                match *operator {
                    "+" => computed_result += operand,
                    "*" => computed_result *= operand,
                    "||" => {
                        computed_result = format!("{}{}", computed_result, operand).parse().unwrap();
                    }
                    _ => panic!("Invalid operator"),
                }

            }

            if computed_result == result {
                final_result += result;
                break;
            }
        }


    }

    final_result
}


fn cartesian_product<T: Clone>(elements: &[T], repeat: usize) -> Vec<Vec<T>> {
    if repeat == 0 {
        return vec![vec![]];
    }

    let mut result: Vec<Vec<T>> = vec![vec![]];
    for _ in 0..repeat {
        let mut temp = Vec::new();
        for seq in &result {
            for elem in elements {
                let mut new_seq = seq.clone();
                new_seq.push(elem.clone());
                temp.push(new_seq);
            }
        }
        result = temp;
    }
    result
}
