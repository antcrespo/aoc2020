
use std::fs;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("./src/input/day4.txt")
        .expect("Something went wrong reading the file");
    let passports = input.split("\r\n\r\n");
    let mut gold = 0;
    let mut silver = 0;
    let req_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    let req_fields_gold = [
        r"byr:(19[2-9][0-9]|200[012])", 
        r"iyr:20(1[0-9]|20)", 
        r"eyr:20(2[0-9]|30)", 
        r"hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)", 
        r"hcl:#([0-9]|[a-f]){6}", 
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)", 
        r"pid:[0-9]{9}"
    ];
    
    let mut gold_regexes = Vec::new();
    let start_bound = r"\b";
    let end_bound = r"\b";
    for field in &req_fields_gold {
        let bounded_field = start_bound.to_owned() + field + end_bound;
        let re: Regex = Regex::new(&bounded_field).unwrap();
        gold_regexes.push(re);
    }
    for passport in passports {
        if valid_silver(&passport, &req_fields) {
            silver += 1;
        }
        if valid_gold(&passport, &gold_regexes) {
            gold += 1;
        }
    }
    println!("Silver: {}", silver);
    println!("Gold: {}", gold);
}

fn valid_silver(passport: &str, req_fields: &[&str]) -> bool {
    for field in req_fields {
        if !passport.contains(field) {
            return false;
        }
    }
    return true;
}

fn valid_gold(passport: &str, req_fields: &Vec<Regex>) -> bool {
    for regex in req_fields {
        if !regex.is_match(passport) {
            // println!("{} failed", regex);
            // println!("passport {}", passport);
            return false;
        }
    }
    return true;
}

