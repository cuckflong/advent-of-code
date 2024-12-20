fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    part_one(&grid);
    part_two(&grid);
}

fn recursive_search(
    grid: &Vec<Vec<usize>>,
    start: (usize, usize),
    cur_num: usize,
) -> Vec<(usize, usize)> {
    let mut nine_points = Vec::new();
    if grid[start.0][start.1] != cur_num {
        return nine_points;
    }
    if grid[start.0][start.1] == 9 {
        nine_points.push(start);
        return nine_points;
    }
    if start.0 > 0 {
        nine_points.extend(recursive_search(grid, (start.0 - 1, start.1), cur_num + 1));
    }
    if start.0 < grid.len() - 1 {
        nine_points.extend(recursive_search(grid, (start.0 + 1, start.1), cur_num + 1));
    }
    if start.1 > 0 {
        nine_points.extend(recursive_search(grid, (start.0, start.1 - 1), cur_num + 1));
    }
    if start.1 < grid[0].len() - 1 {
        nine_points.extend(recursive_search(grid, (start.0, start.1 + 1), cur_num + 1));
    }
    nine_points
}

fn part_one(grid: &Vec<Vec<usize>>) {
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let mut points = recursive_search(grid, (i, j), 0);
                points.sort();
                points.dedup();
                res += points.len();
            }
        }
    }
    println!("{}", res);
}

fn part_two(grid: &Vec<Vec<usize>>) {
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            res += recursive_search(grid, (i, j), 0).len();
        }
    }
    println!("{}", res);
}
