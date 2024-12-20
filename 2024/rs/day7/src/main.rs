fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut values: Vec<i64> = vec![];
    let mut nums: Vec<Vec<i64>> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let value: i64 = parts[0].parse().expect("No int1");
        let n: Vec<i64> = parts[1]
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse().expect("Not int"))
            .collect();
        values.push(value);
        nums.push(n);
    }

    part_one(values.clone(), nums.clone());
    part_two(values, nums);
}

fn part_one(values: Vec<i64>, nums: Vec<Vec<i64>>) {
    let mut sum = 0;
    for i in 0..values.len() {
        let value = values[i];
        let ns = &nums[i];
        if is_possible(0, value, ns.clone()) {
            sum += value;
        }
    }
    println!("Part one: {}", sum);
}

fn part_two(values: Vec<i64>, nums: Vec<Vec<i64>>) {
    let mut sum = 0;
    for i in 0..values.len() {
        let value = values[i];
        let ns = &nums[i];
        if is_possible_2(0, value, ns.clone()) {
            sum += value;
        }
    }
    println!("Part two: {}", sum);
}

fn is_possible(mut cur: i64, value: i64, mut nums: Vec<i64>) -> bool {
    if cur == 0 {
        cur = nums.remove(0);
    }
    let next_val = nums.remove(0);

    let cur_plus = cur + next_val;
    let cur_mul = cur * next_val;

    if nums.len() == 0 {
        if cur_plus == value || cur_mul == value {
            return true;
        } else {
            return false;
        }
    }

    return is_possible(cur_plus, value, nums.clone()) || is_possible(cur_mul, value, nums.clone());
}

fn is_possible_2(mut cur: i64, value: i64, mut nums: Vec<i64>) -> bool {
    if cur == 0 {
        cur = nums.remove(0);
    }
    let next_val = nums.remove(0);

    let cur_plus = cur + next_val;
    let cur_mul = cur * next_val;
    let mut cur_append_str = cur.to_string();
    cur_append_str.push_str(next_val.to_string().as_str());
    let cur_append = cur_append_str.parse::<i64>().expect("Not int");

    if nums.len() == 0 {
        if cur_plus == value || cur_mul == value || cur_append == value {
            return true;
        } else {
            return false;
        }
    }

    return is_possible_2(cur_plus, value, nums.clone())
        || is_possible_2(cur_mul, value, nums.clone())
        || is_possible_2(cur_append, value, nums.clone());
}
