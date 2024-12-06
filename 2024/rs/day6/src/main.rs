#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut current_pos = (0, 0);
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' && ch != '#' {
                current_pos = (row, col);
            }
        }
    }

    let path = part_one(grid.clone(), current_pos);
    part_two(grid.clone(), current_pos, path);
}

fn is_possible(mut grid: Vec<Vec<char>>, mut current_pos: (usize, usize)) -> bool {
    let mut direction;

    if grid[current_pos.0][current_pos.1] == '^' {
        direction = Direction::Up;
    } else if grid[current_pos.0][current_pos.1] == 'v' {
        direction = Direction::Down;
    } else if grid[current_pos.0][current_pos.1] == '<' {
        direction = Direction::Left;
    } else {
        direction = Direction::Right;
    }

    grid[current_pos.0][current_pos.1] = 'X';

    let mut steps = 1;
    let mut visited = std::collections::HashMap::new();
    visited.insert((current_pos, direction.clone()), steps);

    loop {
        match direction {
            Direction::Up => {
                if current_pos.0 == 0 {
                    break;
                } else if grid[current_pos.0 - 1][current_pos.1] == '#' {
                    direction = Direction::Right;
                } else {
                    current_pos.0 -= 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                    }
                }
            }
            Direction::Down => {
                if current_pos.0 == grid.len() - 1 {
                    break;
                } else if grid[current_pos.0 + 1][current_pos.1] == '#' {
                    direction = Direction::Left;
                } else {
                    current_pos.0 += 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                    }
                }
            }
            Direction::Left => {
                if current_pos.1 == 0 {
                    break;
                } else if grid[current_pos.0][current_pos.1 - 1] == '#' {
                    direction = Direction::Up;
                } else {
                    current_pos.1 -= 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                    }
                }
            }
            Direction::Right => {
                if current_pos.1 == grid[0].len() - 1 {
                    break;
                } else if grid[current_pos.0][current_pos.1 + 1] == '#' {
                    direction = Direction::Down;
                } else {
                    current_pos.1 += 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                    }
                }
            }
        }

        if visited.contains_key(&(current_pos, direction.clone())) {
            return false;
        } else {
            visited.insert((current_pos, direction.clone()), steps);
        }
    }

    true
}

fn part_one(mut grid: Vec<Vec<char>>, mut current_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut continue_loop = true;
    let mut direction;

    if grid[current_pos.0][current_pos.1] == '^' {
        direction = Direction::Up;
    } else if grid[current_pos.0][current_pos.1] == 'v' {
        direction = Direction::Down;
    } else if grid[current_pos.0][current_pos.1] == '<' {
        direction = Direction::Left;
    } else {
        direction = Direction::Right;
    }

    grid[current_pos.0][current_pos.1] = 'X';

    let mut steps = 1;
    let mut path = vec![(current_pos)];

    while continue_loop {
        match direction {
            Direction::Up => {
                if current_pos.0 == 0 {
                    continue_loop = false;
                } else if grid[current_pos.0 - 1][current_pos.1] == '#' {
                    direction = Direction::Right;
                } else {
                    current_pos.0 -= 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                        path.push(current_pos);
                    }
                }
            }
            Direction::Down => {
                if current_pos.0 == grid.len() - 1 {
                    continue_loop = false;
                } else if grid[current_pos.0 + 1][current_pos.1] == '#' {
                    direction = Direction::Left;
                } else {
                    current_pos.0 += 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                        path.push(current_pos);
                    }
                }
            }
            Direction::Left => {
                if current_pos.1 == 0 {
                    continue_loop = false;
                } else if grid[current_pos.0][current_pos.1 - 1] == '#' {
                    direction = Direction::Up;
                } else {
                    current_pos.1 -= 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                        path.push(current_pos);
                    }
                }
            }
            Direction::Right => {
                if current_pos.1 == grid[0].len() - 1 {
                    continue_loop = false;
                } else if grid[current_pos.0][current_pos.1 + 1] == '#' {
                    direction = Direction::Down;
                } else {
                    current_pos.1 += 1;
                    if grid[current_pos.0][current_pos.1] == '.' {
                        grid[current_pos.0][current_pos.1] = 'X';
                        steps += 1;
                        path.push(current_pos);
                    }
                }
            }
        }
    }

    println!("Part one: {}", steps);
    path
}

fn part_two(grid: Vec<Vec<char>>, current_pos: (usize, usize), path: Vec<(usize, usize)>) {
    let mut count = 0;
    for pos in path {
        let mut new_grid = grid.clone();
        new_grid[pos.0][pos.1] = '#';
        if !is_possible(new_grid, current_pos) {
            count += 1;
        }
    }

    println!("Part two: {}", count);
}
