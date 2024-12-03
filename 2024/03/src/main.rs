use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    let input_path = "input.txt";
    let file = File::open(&input_path)?;
    let reader = io::BufReader::new(file);

    let mut result_part_1: u64 = 0;
    let mut result_part_2: u64 = 0;

    let mut mul_enabled = true;

    for line in reader.lines() {
        let line = line?;
        result_part_1 += parse_and_multiply(&line);

        let (result, last_mul_enabled) = parse_and_multiply_with_conditions(&line, mul_enabled);
        mul_enabled = last_mul_enabled;

        result_part_2 += result;
    }

    print!("Part-1: {}\n", result_part_1);
    print!("Part-2: {}\n", result_part_2);


    Ok(())
}


fn parse_and_multiply(line: &str) -> u64 {
    let mut result: u64 = 0;
    let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for (_, [a, b])  in regex.captures_iter(line).map(|cap| cap.extract()) {
        let mul =  a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
        result += u64::from(mul);
    }

    result
}

fn parse_and_multiply_with_conditions(line: &str, mul_enabled: bool ) -> (u64, bool) {
    let mut result: u64 = 0;
    let mut do_mul: bool = mul_enabled;
    let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();

    for cap in regex.captures_iter(line) {
        let matched = cap.get(0).unwrap().as_str();

        if matched == "do()" {
            do_mul = true;
        } else if matched == "don't()" {
            do_mul = false;
        } else if matched.starts_with("mul") {
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();

            if do_mul {
                result += u64::from(a*b);
            }

        }
    }
    return (result, do_mul);

}
