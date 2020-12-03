
use std::fs;

fn main() {

    let input = fs::read_to_string("./src/input/day3.txt")
        .expect("Something went wrong reading the file");
    let lines = input.split("\n");
    let mut slope = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for symbol in line.chars() {
            if (symbol == '#') || (symbol == '.') {
                row.push(symbol);
            }
        }
        slope.push(row);
    }
    let silver = trees_hit(3, 1, &slope);
    println!("Silver {}", silver);
    let mut gold = silver;
    gold *= trees_hit(1,1, &slope);
    gold *= trees_hit(5,1, &slope);
    gold *= trees_hit(7,1, &slope);
    gold *= trees_hit(1,2, &slope);
    println!("Gold {}", gold);
}

fn trees_hit(nav_right: usize, nav_down: usize, slope: & Vec<Vec<char>>) -> usize {
    let mut x = 0;
    let mut y = 0;
    let x_len = slope[0].len();
    let y_len = slope.len();
    let mut res = 0;
    // println!("x len {}, y len {}", x_len, y_len);
    while y < y_len {
        let symbol = slope[y][x];
        if symbol == '#' {
            res = res + 1;
        }
        // println!("x {}, y {}, char {}", x, y, symbol);
        x = (x + nav_right) % x_len;
        y = y + nav_down;
    }
    return res;
}