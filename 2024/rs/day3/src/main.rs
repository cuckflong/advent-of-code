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
    let mults_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mults: Vec<_> = mults_re.find_iter(input).collect();

    let dos_re = Regex::new(r"do\(\)").unwrap();
    let dos: Vec<_> = dos_re.find_iter(input).collect();
    let dos_starts: Vec<usize> = dos.iter().map(|d| d.start()).collect();

    let donts_re = Regex::new(r"don't\(\)").unwrap();
    let donts: Vec<_> = donts_re.find_iter(input).collect();
    let donts_starts: Vec<usize> = donts.iter().map(|d| d.start()).collect();

    let mut enabled = true;
    let mut last_mult_start = 0;
    let mut sum = 0;
    for mult in mults {
        let mult_start = mult.start();
        while last_mult_start < mult_start {
            if enabled {
                for dont_start in &donts_starts {
                    if dont_start <= &mult_start && dont_start > &last_mult_start {
                        enabled = false;
                    }
                }
            } else {
                for do_start in &dos_starts {
                    if do_start <= &mult_start && do_start > &last_mult_start {
                        enabled = true;
                    }
                }
            }
            last_mult_start += 1;
        }
        if enabled {
            sum += parse_mul_strings(&mult.as_str().to_string());
        }
    }
    println!("{}", sum);
}
