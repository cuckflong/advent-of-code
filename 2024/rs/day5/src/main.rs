use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Side {
    left: HashSet<i32>,
    right: HashSet<i32>,
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let split_pos = lines.iter().position(|&line| line == "").unwrap();
    let rules = &lines[..split_pos];
    let updates: Vec<Vec<i32>> = lines[split_pos + 1..]
        .iter()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut rule_map: HashMap<i32, Side> = HashMap::new();

    for rule in rules {
        let parts: Vec<&str> = rule.split('|').collect();
        let left_num = parts[0].parse::<i32>().unwrap();
        let right_num = parts[1].parse::<i32>().unwrap();

        let mut right_set = HashSet::new();
        right_set.insert(right_num);
        rule_map
            .entry(left_num)
            .and_modify(|side| {
                side.right.insert(right_num);
            })
            .or_insert(Side {
                left: HashSet::new(),
                right: right_set,
            });

        let mut left_set = HashSet::new();
        left_set.insert(left_num);
        rule_map
            .entry(right_num)
            .and_modify(|side| {
                side.left.insert(left_num);
            })
            .or_insert(Side {
                left: left_set,
                right: HashSet::new(),
            });
    }

    part_one(&updates.to_vec(), &rule_map);
    part_two(&updates.to_vec(), &rule_map);
}

fn is_update_valid(update: &Vec<i32>, rule_map: &HashMap<i32, Side>) -> bool {
    let mut valid = true;
    for i in 0..update.len() {
        let mut left_set: HashSet<i32> = HashSet::new();
        let mut right_set: HashSet<i32> = HashSet::new();
        if i > 0 {
            left_set = update[..i].iter().cloned().collect();
        }
        if i < update.len() - 1 {
            right_set = update[i + 1..].iter().cloned().collect();
        }

        let current_num = update[i];
        let current_rules = rule_map.get(&current_num).unwrap();
        let left_rules = &current_rules.left;
        let right_rules = &current_rules.right;

        for &left_rule in left_rules {
            if right_set.contains(&left_rule) {
                valid = false;
                break;
            }
        }

        for &right_rule in right_rules {
            if left_set.contains(&right_rule) {
                valid = false;
                break;
            }
        }

        if !valid {
            break;
        }
    }
    valid
}

fn part_one(updates: &Vec<Vec<i32>>, rule_map: &HashMap<i32, Side>) {
    let mut sum = 0;
    for update in updates {
        if is_update_valid(update, rule_map) {
            sum += update[update.len() / 2];
        }
    }
    println!("{}", sum);
}

fn part_two(updates: &Vec<Vec<i32>>, rule_map: &HashMap<i32, Side>) {
    let mut sum = 0;
    for update in updates {
        if is_update_valid(update, rule_map) {
            continue;
        }
        let mut new_update = update.clone();
        while !is_update_valid(&new_update, rule_map) {
            for i in 0..new_update.len() - 1 {
                if !is_update_valid(&new_update[i..i + 2].to_vec(), rule_map) {
                    new_update.swap(i, i + 1);
                }
            }
        }
        sum += new_update[new_update.len() / 2];
    }
    println!("{}", sum);
}
