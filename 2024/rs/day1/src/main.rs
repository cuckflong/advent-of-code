fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let binding = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let lines = binding.lines().collect::<Vec<&str>>();

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in lines {
        let number = line.split_whitespace().collect::<Vec<&str>>();
        left_numbers.push(number[0].parse::<i32>().unwrap());
        right_numbers.push(number[1].parse::<i32>().unwrap());
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut total_distance = 0;

    for i in 0..left_numbers.len() {
        let distance = (left_numbers[i] - right_numbers[i]).abs();
        total_distance += distance;
    }

    println!("{}", total_distance);
}

fn part_two() {
    let mut score_cache: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    let binding = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let lines = binding.lines().collect::<Vec<&str>>();

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in lines {
        let number = line.split_whitespace().collect::<Vec<&str>>();
        left_numbers.push(number[0].parse::<i32>().unwrap());
        right_numbers.push(number[1].parse::<i32>().unwrap());
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut total_score = 0;
    let mut right_index = 0;

    for i in 0..left_numbers.len() {
        if let Some(cached_score) = score_cache.get(&left_numbers[i]) {
            total_score += cached_score;
            continue;
        }
        let mut count = 0;
        while right_index < right_numbers.len() && left_numbers[i] >= right_numbers[right_index] {
            if left_numbers[i] == right_numbers[right_index] {
                count += 1;
            }
            right_index += 1;
        }
        let score = left_numbers[i] * count;
        score_cache.insert(left_numbers[i], score);
        total_score += score;
    }

    println!("{}", total_score);
}
