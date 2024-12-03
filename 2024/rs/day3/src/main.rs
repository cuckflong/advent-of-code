use regex::Regex;

fn main() {
    let input =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part_one(&input);
    part_two(&input);
}

fn parse_mul_strings(input: &str) -> i32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let matches: Vec<_> = re.find_iter(input).collect();
    let numbers: Vec<i32> = matches
        .iter()
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    numbers[0] * numbers[1]
}

fn part_one(input: &str) {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<_> = re.find_iter(input).collect();
    let mul_strings: Vec<String> = matches.iter().map(|m| m.as_str().to_string()).collect();
    let mut sum = 0;
    for mul_string in mul_strings {
        sum += parse_mul_strings(&mul_string);
    }
    println!("{}", sum);
}

fn part_two(input: &str) {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let matches: Vec<_> = re.find_iter(input).collect();

    let mut enabled = true;
    let mut sum = 0;

    for match_ in matches {
        if match_.as_str() == "do()" {
            enabled = true;
        } else if match_.as_str() == "don't()" {
            enabled = false;
        } else {
            if enabled {
                sum += parse_mul_strings(&match_.as_str().to_string());
            }
        }
    }

    println!("{}", sum);
}
