fn main() {
    part_one();
    part_two();
}

fn is_safe(report: &Vec<i32>) -> bool {
    let is_increasing = report[0] < report[1];
    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];
        if is_increasing && diff > 0 {
            return false;
        }
        if !is_increasing && diff < 0 {
            return false;
        }
        if diff == 0 {
            return false;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn part_one() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split(' ')
            .map(|n| n.parse().expect("Failed to parse number"))
            .collect();
        reports.push(numbers);
    }

    let mut safe_count = 0;

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
}

fn part_two() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split(' ')
            .map(|n| n.parse().expect("Failed to parse number"))
            .collect();
        reports.push(numbers);
    }

    let mut safe_count = 0;

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut modified = report.clone();
                modified.remove(i);
                if is_safe(&modified) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Safe count: {}", safe_count);
}
