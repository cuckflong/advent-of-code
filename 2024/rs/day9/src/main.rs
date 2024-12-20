fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let numbers: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let blocks = to_blocks(numbers);
    part_one(blocks.clone());
    part_two(blocks.clone());
}

fn to_blocks(numbers: Vec<i64>) -> Vec<i64> {
    let mut blocks = Vec::new();
    let mut is_file = true;
    let mut id = 0;
    for number in numbers {
        if is_file {
            blocks.extend(vec![id; number.try_into().unwrap()]);
            id += 1;
        } else {
            blocks.extend(vec![-1; number.try_into().unwrap()]);
        }
        is_file = !is_file;
    }
    blocks
}

fn calc_checksum(blocks: Vec<i64>) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != -1)
        .map(|(i, &x)| i as i64 * x)
        .sum()
}

fn part_one(mut blocks: Vec<i64>) {
    let mut i = 0;
    let mut j = blocks.len() - 1;
    while i < j {
        if blocks[j] == -1 {
            j -= 1;
        }
        if blocks[i] != -1 {
            i += 1;
        }
        if blocks[i] == -1 && blocks[j] != -1 {
            blocks[i] = blocks[j];
            blocks[j] = -1;
            i += 1;
            j -= 1;
        }
    }
    println!("{:?}", calc_checksum(blocks));
}

fn part_two(mut blocks: Vec<i64>) {
    let mut id = blocks.iter().max().unwrap().clone();
    while id != 0 {
        let mut file_index = 0;
        let mut file_length = 0;
        for j in (0..blocks.len()).rev() {
            if blocks[j] == id {
                file_index = j;
                file_length += 1;
            }
            if file_length != 0 && blocks[j] != id {
                break;
            }
        }
        let mut free_index = 0;
        let mut free_count = 0;
        for i in 0..blocks.len() {
            if blocks[i] == id {
                free_index = 0;
                break;
            }
            if blocks[i] == -1 {
                if free_count == 0 {
                    free_index = i;
                }
                free_count += 1;
            } else {
                free_count = 0;
                free_index = 0;
            }
            if free_count >= file_length {
                break;
            }
        }

        if free_index != 0 {
            for i in 0..file_length {
                blocks[free_index + i] = id;
                blocks[file_index + i] = -1;
            }
        }

        id -= 1;
    }
    println!("{:?}", calc_checksum(blocks));
}
