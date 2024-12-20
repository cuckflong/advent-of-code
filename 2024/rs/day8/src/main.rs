use std::collections::HashMap;

fn main() {
    let contents =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    print_grid(&grid);
    let map = collect_antennas(&grid);
    part_one(&grid, &map);
    part_two(&grid, &map);
}

fn collect_antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas = HashMap::new();

    for (row, line) in grid.iter().enumerate() {
        for (column, &cell) in line.iter().enumerate() {
            if cell != '.' {
                antennas
                    .entry(cell)
                    .or_insert_with(Vec::new)
                    .push((column as i32, row as i32));
            }
        }
    }

    antennas
}

fn calc_antidotes(
    max_x: i32,
    max_y: i32,
    point_a: (i32, i32),
    point_b: (i32, i32),
    extend: bool,
) -> Vec<(i32, i32)> {
    let mut antidotes = Vec::new();

    let diff_x = point_a.0 - point_b.0;
    let diff_y = point_a.1 - point_b.1;
    let mut antidote_a = (point_a.0 + diff_x, point_a.1 + diff_y);
    let mut antidote_b = (point_b.0 - diff_x, point_b.1 - diff_y);

    if extend {
        antidotes.push(point_a);
        antidotes.push(point_b);
    }

    if antidote_a.0 >= 0 && antidote_a.1 >= 0 && antidote_a.0 < max_x && antidote_a.1 < max_y {
        antidotes.push(antidote_a);
    }

    if extend {
        while antidote_a.0 >= 0 && antidote_a.1 >= 0 && antidote_a.0 < max_x && antidote_a.1 < max_y
        {
            antidotes.push(antidote_a);
            antidote_a = (antidote_a.0 + diff_x, antidote_a.1 + diff_y);
        }
    }

    if antidote_b.0 >= 0 && antidote_b.1 >= 0 && antidote_b.0 < max_x && antidote_b.1 < max_y {
        antidotes.push(antidote_b);
    }

    if extend {
        while antidote_b.0 >= 0 && antidote_b.1 >= 0 && antidote_b.0 < max_x && antidote_b.1 < max_y
        {
            antidotes.push(antidote_b);
            antidote_b = (antidote_b.0 - diff_x, antidote_b.1 - diff_y);
        }
    }

    antidotes
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn part_one(grid: &Vec<Vec<char>>, map: &HashMap<char, Vec<(i32, i32)>>) {
    let mut grid_clone = grid.clone();
    for (_, points) in map {
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let antidotes = calc_antidotes(
                    grid.len().try_into().unwrap(),
                    grid[0].len().try_into().unwrap(),
                    points[i],
                    points[j],
                    false,
                );
                for antidote in antidotes {
                    grid_clone[antidote.1 as usize][antidote.0 as usize] = '#';
                }
            }
        }
    }
    let count = grid_clone
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum::<usize>();
    println!("Number of antidotes: {}", count);
    print_grid(&grid_clone);
}

fn part_two(grid: &Vec<Vec<char>>, map: &HashMap<char, Vec<(i32, i32)>>) {
    let mut grid_clone = grid.clone();
    for (_, points) in map {
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let antidotes = calc_antidotes(
                    grid.len().try_into().unwrap(),
                    grid[0].len().try_into().unwrap(),
                    points[i],
                    points[j],
                    true,
                );
                for antidote in antidotes {
                    grid_clone[antidote.1 as usize][antidote.0 as usize] = '#';
                }
            }
        }
    }
    let count = grid_clone
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum::<usize>();
    println!("Number of antidotes: {}", count);
    print_grid(&grid_clone);
}
