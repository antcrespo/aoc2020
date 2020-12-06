
use std::fs;

fn main() {

    let input = fs::read_to_string("./src/input/day6.txt")
        .expect("Something went wrong reading the file");
    let declarations = input.split("\r\n\r\n");
    let mut silver = 0;
    let mut gold = 0;
    for declaration in declarations {
        silver += count_yes_silver(declaration);
        gold += count_yes_gold(declaration);
    }
    println!("Silver {}", silver);
    println!("Gold {}", gold);
    
}

fn count_yes_silver(declaration: &str) -> i32 {
    let mut yes: [bool; 26] = [false; 26];
    let a = 'a' as i32;
    for symbol in declaration.chars() {
        let idx = (symbol as i32) - a;
        if idx >= 0 && idx < 26 {
            yes[idx as usize] = true;
        }
    }
    let mut res = 0;
    for val in &yes {
        if *val {
            res += 1;
        }
    }
    return res;
}

fn count_yes_gold(declaration: &str) -> i32 {
    let mut yes: [bool; 26] = [true; 26];
    let a = 'a' as i32;
    let indivs = declaration.split("\r\n");

    for indiv in indivs {
        let mut my_yes: [bool; 26] = [false; 26];
        for symbol in indiv.chars() {
            let idx = (symbol as i32) - a;
            if idx >= 0 && idx < 26 {
                my_yes[idx as usize] = true;
            }
        }
        for (i, answer) in (&my_yes).iter().enumerate() {
            yes[i] &= answer;
        }
    }

    
    let mut res = 0;
    for val in &yes {
        if *val {
            res += 1;
        }
    }
    return res;
}
