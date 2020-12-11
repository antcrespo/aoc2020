
use std::fs;
use std::collections::HashMap;

fn main() {

    let input = fs::read_to_string("./src/input/day10.txt")
        .expect("Something went wrong reading the file");
    let mut numbers = Vec::new();
    let mut max = 0;
    let mut ways: HashMap<i64, i64> = HashMap::new();
    for line in input.split("\r\n") {
        // println!("{}", line);
        let value = line.parse::<i64>().unwrap();
        if value > max {
            max = value;
        }
        ways.insert(value, 0);
        numbers.push(value);
    }
    numbers.sort();
    let mut jolts = 0;
    let mut ones = 0;
    let mut threes = 1;
    for number in &numbers {
        let diff = number - jolts;
        if diff == 3 {
            threes += 1;
        } else if diff == 1 {
            ones += 1;
        }
        jolts = *number;
    }
    println!("Silver: {}", ones * threes);

    ways.insert(0,1);
    for i in 1..=max {
        if ways.contains_key(&i) {
            let mut ways_here: i64 = 0;
            ways_here += ways.get(&(i-1)).unwrap_or(&0);
            ways_here += ways.get(&(i-2)).unwrap_or(&0);
            ways_here += ways.get(&(i-3)).unwrap_or(&0);
            ways.insert(i, ways_here);
        }
    }
    println!("Gold: {}", ways.get(&max).unwrap());
}

