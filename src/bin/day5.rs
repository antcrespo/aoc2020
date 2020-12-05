
use std::fs;
use std::collections::HashSet;

fn main() {

    let input = fs::read_to_string("./src/input/day5.txt")
        .expect("Something went wrong reading the file");
    let passes = input.split("\r\n");
    
    let mut silver = 0;
    let mut seen = HashSet::new();

    for pass in passes {
       let row = range_search(&pass, 0, 127, 'F', 'B');
       let col = range_search(&pass, 0, 7, 'L', 'R');
       let id = seat_id(row, col);
       if id > silver {
           silver = id;
       }
       seen.insert(id);
    }
    println!("Silver: {}", silver);
    let mut i = 1;
    loop {
        let prev = i-1;
        let next = i +1;
        if !seen.contains(&i) && seen.contains(&prev) && seen.contains(&next) {
            println!("Gold: {}", i);
            break;
        }
        i += 1;
    }
}

fn seat_id(row: i32, col: i32) -> i32 {
    return (row * 8) + col;
}

fn range_search(pass: &str, bottom: i32, top: i32, lower: char, upper: char) -> i32 {
    let mut max = top;
    let mut min = bottom;
    for symbol in pass.chars() {
        let mid = min + (max-min)/2;
        if symbol == lower {
            max = mid;
        } else if symbol == upper {
            min = mid + 1;
        } else {
            continue;
        }
        // println!("{}-{}", min, max);
    }
    return min;
}

