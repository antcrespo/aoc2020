
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use regex::Regex;

struct Holding {
    color: String,
    count: i32,
}

fn main() {

    let input = fs::read_to_string("./src/input/day7.txt")
        .expect("Something went wrong reading the file");
    let rule_regex: Regex = Regex::new(r"\b([a-z ]+) bags? contain ([a-z0-9 ,]+)\.").unwrap();
    let bag_regex: Regex = Regex::new(r"(\d*) ([a-z ]+) bags?").unwrap();
    let mut holdings_map: HashMap<String, Vec<Holding>> = HashMap::new();
    let mut holds_map: HashMap<String, Vec<Holding>> = HashMap::new();
    for rule in rule_regex.captures_iter(&input) {
        let holding_color = &rule[1].to_string();
        // println!("{}:", holding_color);
        let holdings = &rule[2].to_string();
        for holding in bag_regex.captures_iter(&holdings) {
            let inner_color = &holding[2].to_string();
            let inner_count = &holding[1].to_string();
            // println!("{} {}", inner_count, inner_color);
            let parsed_count = if inner_count == "" { 0 } else { inner_count.parse().unwrap() };
            if parsed_count == 0 {
                continue;
            }

            let holding: Holding = Holding {color: holding_color.to_string(), count: parsed_count};
            if holdings_map.contains_key(inner_color) {
                let parents = holdings_map.get_mut(inner_color).unwrap();
                parents.push(holding);
            } else {
                let mut new_list = Vec::new();
                new_list.push(holding);
                holdings_map.insert(inner_color.to_string(), new_list);
            }

            let outer_holding: Holding = Holding {color: inner_color.to_string(), count: parsed_count};
            if holds_map.contains_key(holding_color) {
                let children = holds_map.get_mut(holding_color).unwrap();
                children.push(outer_holding);
            } else {
                let mut new_list = Vec::new();
                new_list.push(outer_holding);
                holds_map.insert(holding_color.to_string(), new_list);
            }
        }
    }

    let mut seen = HashSet::new();
    let start = "shiny gold";
    let mut to_visit = LinkedList::new();
    to_visit.push_back(start);
    while to_visit.len() > 0 {
        let next = to_visit.pop_front().unwrap();
        if holdings_map.contains_key(next) {
            let holds = holdings_map.get(next).unwrap();
            for hold in holds {
                if !seen.contains(&hold.color) {
                    to_visit.push_back(&hold.color);
                    seen.insert(hold.color.clone());
                }
            }
        }
    }
    println!("Silver: {}", seen.len());
    println!("Gold: {}", count_bags(&holds_map, &start));

}

fn count_bags(holds_map: &HashMap<String, Vec<Holding>>, color: &str) -> i32 {
    if holds_map.contains_key(color) {
        let holds = holds_map.get(color).unwrap();
        let mut sum = 0;
        for hold in holds {
            sum += hold.count + (hold.count * count_bags(&holds_map, &hold.color));
        }
        return sum;
    } else {
        return 0;
    }
}
