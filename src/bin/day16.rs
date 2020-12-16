use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;
use regex::Regex;

#[derive(Debug)]
struct Range {
    low: i32,
    high: i32,
}

#[derive(Debug)]
struct Field {
    name: String,
    ranges: [Range; 2],
}

fn main() {

    let input = fs::read_to_string("./src/input/day16.txt")
        .expect("Something went wrong reading the file");
    let field_reg: Regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    
    let your_reg: Regex = Regex::new(r"your ticket:\r\n(\d+(,\d+)+)").unwrap();
    
    let mut fields = Vec::new();
    let mut possibilities: Vec<HashSet<String>> = Vec::new();
    let mut names = Vec::new();
    for raw_field in field_reg.captures_iter(&input) {
        let field = field_from_input(raw_field);
        names.push(field.name.clone());
        fields.push(field);
    }
    let possible_names = HashSet::from_iter(names);
    for _ in &fields {
        possibilities.push(possible_names.clone());
    }

    let sections: Vec<&str> = input.split("nearby tickets:\r\n").collect();
    let nearby_lines = sections[1].split("\r\n");
    let mut error_rate = 0;
    for line in nearby_lines {
        let values: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        let (ticket_rate, total_errors) = ticket_errors(&values, &fields);
        error_rate += ticket_rate;
        if total_errors == 0 {
            for (value, possible_names) in values.iter().zip(possibilities.iter_mut()) {
                let invalids = invalid_fields(*value, &fields);
                for invalid in invalids {
                    possible_names.remove(&invalid);
                }
            }
        }
    }
    println!("Silver: {}", error_rate);
    let mut changed = true;
    let mut solved = HashSet::new();
    while changed {
        changed = false;
        for name_set in possibilities.iter_mut() {
            if name_set.len() == 1 {
                for name in name_set.iter() {
                    changed |= solved.insert(name.clone());
                }
            } else {
                for name in &solved {
                    changed |= name_set.remove(name.as_str());
                }
            }
        }
    }
    let final_possibilities: Vec<String> = possibilities.iter_mut().map(|x| x.drain().last().unwrap()).collect();
    
    let my_ticket = your_reg.captures(&sections[0]).unwrap().get(1).unwrap().as_str();
    let my_values: Vec<i32> = my_ticket.split(",").map(|x| x.parse().unwrap()).collect();
    let mut gold: i64 = 1;
    for (i, field_name) in final_possibilities.iter().enumerate() {
        if field_name.starts_with("departure") {
            gold *= my_values[i] as i64;
        }

    }
    println!("Gold: {}", gold);
}

fn field_from_input(raw_field: regex::Captures) -> Field {
    let name = &raw_field[1];
    let low_range_min = raw_field[2].parse().unwrap();
    let low_range_max = raw_field[3].parse().unwrap();
    let top_range_min = raw_field[4].parse().unwrap();
    let top_range_max = raw_field[5].parse().unwrap();
    let low_range = Range {low: low_range_min, high: low_range_max};
    let top_range = Range {low: top_range_min, high: top_range_max};

    return Field {name: name.to_string(), ranges: [low_range, top_range]};
}

fn in_range(val: i32, ranges: &[Range]) -> bool {
    for range in ranges {
        if val >= range.low && val <= range.high {
            return true;
        }
    }
    return false;
}

fn valid_value(val: i32, fields: &[Field]) -> bool {
    for field in fields {
        if in_range(val, &field.ranges) {
            return true;
        }
    }
    return false;
}

fn ticket_errors(values: &[i32], fields: &[Field]) -> (i32, i32) {
    let mut error_rate = 0;
    let mut error_count = 0;
    for value in values {
        if !valid_value(*value, &fields) {
            error_rate += value;
            error_count += 1;
        }
    }
    return (error_rate, error_count);
}

fn invalid_fields(value: i32, fields: &[Field]) -> Vec<String> {
    let mut invalids = Vec::new();
    for field in fields {
        if !in_range(value, &field.ranges) {
            invalids.push(field.name.clone());
        }
    }
    return invalids;
}