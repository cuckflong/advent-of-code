fn main() {
    let input =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    part_one(grid.clone());
    part_two(grid.clone());
}

fn search_xmas(grid: Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if grid[x][y] != 'X' {
        return 0;
    }

    let mut total = 0;

    if x + 3 < grid.len() {
        if grid[x + 1][y] == 'M' && grid[x + 2][y] == 'A' && grid[x + 3][y] == 'S' {
            total += 1;
        }
    }

    if y + 3 < grid[0].len() {
        if grid[x][y + 1] == 'M' && grid[x][y + 2] == 'A' && grid[x][y + 3] == 'S' {
            total += 1;
        }
    }

    if x >= 3 {
        if grid[x - 1][y] == 'M' && grid[x - 2][y] == 'A' && grid[x - 3][y] == 'S' {
            total += 1;
        }
    }

    if y >= 3 {
        if grid[x][y - 1] == 'M' && grid[x][y - 2] == 'A' && grid[x][y - 3] == 'S' {
            total += 1;
        }
    }

    if x + 3 < grid.len() && y + 3 < grid[0].len() {
        if grid[x + 1][y + 1] == 'M' && grid[x + 2][y + 2] == 'A' && grid[x + 3][y + 3] == 'S' {
            total += 1;
        }
    }

    if x >= 3 && y >= 3 {
        if grid[x - 1][y - 1] == 'M' && grid[x - 2][y - 2] == 'A' && grid[x - 3][y - 3] == 'S' {
            total += 1;
        }
    }

    if x + 3 < grid.len() && y >= 3 {
        if grid[x + 1][y - 1] == 'M' && grid[x + 2][y - 2] == 'A' && grid[x + 3][y - 3] == 'S' {
            total += 1;
        }
    }

    if x >= 3 && y + 3 < grid[0].len() {
        if grid[x - 1][y + 1] == 'M' && grid[x - 2][y + 2] == 'A' && grid[x - 3][y + 3] == 'S' {
            total += 1;
        }
    }

    total
}

fn search_cross(grid: Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if grid[x][y] != 'A' {
        return false;
    }

    if x == 0 || x == grid.len() - 1 || y == 0 || y == grid[0].len() - 1 {
        return false;
    }

    if grid[x - 1][y - 1] == 'A'
        || grid[x + 1][y + 1] == 'A'
        || grid[x + 1][y - 1] == 'A'
        || grid[x - 1][y + 1] == 'A'
    {
        return false;
    }

    if grid[x - 1][y - 1] == 'X'
        || grid[x + 1][y + 1] == 'X'
        || grid[x + 1][y - 1] == 'X'
        || grid[x - 1][y + 1] == 'X'
    {
        return false;
    }

    if grid[x - 1][y - 1] == grid[x + 1][y + 1] || grid[x + 1][y - 1] == grid[x - 1][y + 1] {
        return false;
    }

    true
}

fn part_one(grid: Vec<Vec<char>>) {
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            total += search_xmas(grid.clone(), i, j);
        }
    }

    println!("{}", total);
}

fn part_two(grid: Vec<Vec<char>>) {
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if search_cross(grid.clone(), i, j) {
                total += 1;
            }
        }
    }

    println!("{}", total);
}
