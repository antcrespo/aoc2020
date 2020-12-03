
use std::fs;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("./src/input/day2.txt")
        .expect("Something went wrong reading the file");
    let re: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]+): ([a-z]+)").unwrap();
    
    let mut silver = 0;
    let mut gold = 0;
    for cap in re.captures_iter(&input) {
        let min: u32 = (&cap[1]).parse().unwrap();
        let max: u32 = (&cap[2]).parse().unwrap();
        let character = &cap[3];
        let pass = &cap[4];

        let count: u32 = pass.matches(character).count() as u32;
        if (count <= max) && (count >= min) {
            silver += 1;
        }
        let nth = character.chars().nth(0).unwrap();
        let first = pass.chars().nth((min-1) as usize).unwrap();
        let second = pass.chars().nth((max-1) as usize).unwrap();
        if (first == nth) ^ (second == nth) {
            gold += 1;
        }
    }
    println!("Silver {}", silver);
    println!("Gold {}", gold);
}