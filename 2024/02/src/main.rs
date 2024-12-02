use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let input_path = "input.txt";
    let file = File::open(&input_path)?;
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        reports.push(numbers);
    }

    let mut safe_report_count: i32 = 0;

    for report in reports {
        if is_safe(&report) || is_safe_v2(&report) {
            safe_report_count += 1;
        }
    }

    print!("{}", safe_report_count);

    Ok(())
}


fn is_safe(report: &Vec<i32>) -> bool {
    const MIN_DIFF: i32 = 1;
    const MAX_DIFF: i32 = 3;

    let increasing = report[0] > report[1];

    for i in 1..report.len() {
        if increasing && report[i - 1] < report[i] {
            return false;
        } else if !increasing && report[i - 1] > report[i] {
            return false;
        }
        let absolute_diff = (report[i - 1] - report[i]).abs();
        if absolute_diff < MIN_DIFF || absolute_diff > MAX_DIFF {
            return false;
        }
    }

    return true;
}

// In V2, a report is considered safe if it becomes save by removing
fn is_safe_v2(report: &Vec<i32>) -> bool {
    let mut report_copy = report.clone();
    for i in 0..report.len() {
        report_copy.remove(i);
        if is_safe(&report_copy) {
            return true;
        }
        report_copy.insert(i, report[i]);
    }
    return false;
}
