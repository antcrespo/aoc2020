
use std::fs;

fn main() {

    let input = fs::read_to_string("./src/input/day11.txt")
        .expect("Something went wrong reading the file");
    let mut grid = Vec::new();
    let mut grid2 = Vec::new();
    for line in input.split("\r\n") {
        let mut row = Vec::new();
        let mut row2 = Vec::new();
        for symbol in line.chars() {
            row.push(symbol);
            row2.push(symbol);
        }
        grid.push(row);
        grid2.push(row2);
    }
    let mut changed = true;
    while changed {
        let (new_grid, change) = update_seats(&grid, 4, count_neighbors);
        grid = new_grid;
        changed = change;
    }
    println!("Silver: {}", count_occupied(&grid));
    changed = true;
    while changed {
        let (new_grid, change) = update_seats(&grid2, 5, count_neighbors_gold);
        grid2 = new_grid;
        changed = change;
    }
    println!("Gold: {}", count_occupied(&grid2));
}

fn update_seats(grid: &Vec<Vec<char>>, limit: i32, neighbor_count: fn(&Vec<Vec<char>>, usize, usize) -> i32) -> (Vec<Vec<char>>, bool) {
    let mut new_grid = Vec::new();
    let mut changed = false;
    for (i, row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();
        for (j, symbol) in row.iter().enumerate() {
            if *symbol == '.' {
                new_row.push('.');
                continue;
            }
            let near = neighbor_count(&grid, i, j);
            if *symbol == 'L' {
                if near == 0 {
                    changed = true;
                    new_row.push('#');
                } else {
                    new_row.push('L');
                }
            } else {
                if near < limit {
                    new_row.push('#');
                } else {
                    changed = true;
                    new_row.push('L');
                }
            }
        }
        new_grid.push(new_row);
    }
    return (new_grid, changed);
}

fn count_occupied(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in grid.iter() {
        for symbol in row.iter() {
            if *symbol == '#' {
                count += 1;
            }
        }
    }
    return count;
}

fn count_neighbors(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    let len = grid.len();
    let width = grid[i].len();
    let top_safe = i > 0;
    let bottom_safe = i < len - 1;
    let left_safe = j > 0;
    let right_safe = j < width - 1;
    if top_safe && grid[i-1][j] == '#' {
        count += 1;
    }
    if bottom_safe && grid[i+1][j] == '#' {
        count += 1;
    }
    if left_safe && grid[i][j-1] == '#' {
        count += 1;
    }
    if right_safe && grid[i][j+1] == '#' {
        count += 1;
    }
    if top_safe && left_safe && grid[i-1][j-1] == '#' {
        count +=1;
    }
    if top_safe && right_safe && grid[i-1][j+1] == '#' {
        count +=1;
    }
    if bottom_safe && left_safe && grid[i+1][j-1] == '#' {
        count +=1;
    }
    if bottom_safe && right_safe && grid[i+1][j+1] == '#' {
        count +=1;
    }
    return count;
}

fn count_neighbors_gold(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    count += line_sight(grid, i, j, 0, 1);
    count += line_sight(grid, i, j, 0, -1);
    count += line_sight(grid, i, j, 1, 0);
    count += line_sight(grid, i, j, -1, 0);
    count += line_sight(grid, i, j, 1, 1);
    count += line_sight(grid, i, j, 1, -1);
    count += line_sight(grid, i, j, -1, 1);
    count += line_sight(grid, i, j, -1, -1);
   
    return count;
}

fn line_sight(grid: &Vec<Vec<char>>, i: usize, j: usize, step_i: i32, step_j: i32) -> i32 {
    let len = grid.len() as i32;
    let width = grid[i].len() as i32;
    let mut x = (i as i32) + step_i;
    let mut y = (j as i32) + step_j;
    while x >= 0 && y >= 0 && x < len && y < width {
        if grid[x as usize][y as usize] == '#' {
            return 1;
        } else if grid[x as usize][y as usize] == 'L' {
            return 0;
        }
        x += step_i;
        y += step_j;
    }
    return 0;
}