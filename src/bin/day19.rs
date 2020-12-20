use std::fs;
use std::collections::HashMap;

use regex::Regex;


struct Constant {
    value: char,
}

struct And {
    left: Box<Rule>,
    right: Box<Rule>,
}

struct Dupe {
    rule: Box<Rule>,
}

struct Or {
    left: Box<Rule>,
    right: Box<Rule>,
}

enum Rule {
    Const(Constant),
    And(And),
    Or(Or),
    Dupe(Dupe),
}

fn into_regex(rule: &Rule) -> String {
    match rule {
        Rule::Const(constant) => (constant.value.to_string()),
        Rule::Or(or) => {
            let mut string = "(?:".to_string();
            string.push_str(&into_regex(&*or.left));
            string.push_str("|");
            string.push_str(&into_regex(&*or.right));
            string.push_str(")");
            return string;
        },
        Rule::And(and) => {
            let mut string = into_regex(&*and.left);
            string.push_str(&into_regex(&*and.right));
            return string;
        },
        Rule::Dupe(dupe) => {
            return into_regex(&*dupe.rule);
        }
    }
}

fn main() {

    let input = fs::read_to_string("./src/input/day19.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.split("\r\n\r\n").collect();
    let raw_rules = lines[0];
    let possible_matches = lines[1];
    let mut rule_map = HashMap::new();
    for line in raw_rules.split("\r\n") {
        let rule: Vec<&str> = line.split(":").collect();
        let rule_num = rule[0].parse().expect("Invalid rule number");
        rule_map.insert(rule_num, rule[1].to_string());
    }
    
    let rule = rule_map.get(&0).expect("No rule 0");
    let parsed_rule = parse_rule(&rule_map, rule);
    let mut silver_rule = r"\b".to_string();
    silver_rule.push_str(&into_regex(&parsed_rule));
    silver_rule.push_str(r"\b");
    let silver_regex = Regex::new(&silver_rule).unwrap();
    let silver_finds = silver_regex.find_iter(possible_matches);
    let mut silver = 0;
    for _ in silver_finds {
        silver += 1;
    }

    let rule_42 = rule_map.get(&42).expect("No rule 42");
    let parsed_42 = parse_rule(&rule_map, rule_42);
    let regex_42 = into_regex(&parsed_42);
    let rule_31 = rule_map.get(&31).expect("No rule 31");
    let parsed_31 = parse_rule(&rule_map, rule_31);
    let regex_31 = into_regex(&parsed_31);

    let mut rule_8_gold = regex_42.clone();
    rule_8_gold.push_str("+");

    let rule_11_gold = rule_11_hack(&regex_31, &regex_42, 4);
    let mut gold_rule = r"\b".to_string();
    gold_rule.push_str(&rule_8_gold);
    gold_rule.push_str(&rule_11_gold);
    gold_rule.push_str(r"\b");



    let gold_regex = Regex::new(&gold_rule).unwrap();
    let gold_finds = gold_regex.find_iter(possible_matches);
    let mut gold = 0;
    for _ in gold_finds {
        gold += 1;
    }

    println!("silver regex len {}", silver_rule.len());
    println!("Silver: {}", silver);
    println!("gold regex len {}", gold_rule.len());
    println!("Gold: {}", gold);

}
fn rule_11_hack(rule_31: &str, rule_42: &str, n: u32) -> String {
    let mut rule = "(?:".to_string();
    for i in 1..=n {
        let tmp = format!("{}{{{}}}{}{{{}}}", rule_42, i, rule_31, i);
        rule.push_str(&tmp);
        if i != n {
            rule.push_str("|");
        }
    }
    rule.push_str(")");
    return rule;
}
fn parse_rule(rule_map: &HashMap<i32, String>, rule: &str) -> Rule {
    if rule.contains('a') {
        return Rule::Const(Constant {value: 'a'});
    } else if rule.contains('b') {
        return Rule::Const(Constant {value: 'b'});
    } else if rule.contains("|") {
        //or
        let parts: Vec<&str> = rule.split("|").collect();
        return Rule::Or(Or {left: Box::new(parse_rule(rule_map, parts[0])), right: Box::new(parse_rule(rule_map, parts[1]))});
    } else if rule.contains(" ") {
        //and or dupe
        let parts: Vec<&str> = rule.split_ascii_whitespace().collect();
        if parts.len() == 1 {
            return Rule::Dupe(Dupe {rule: Box::new(parse_rule(rule_map, parts[0]))});
        } else {
            return Rule::And(And {left: Box::new(parse_rule(rule_map, parts[0])), right: Box::new(parse_rule(rule_map, parts[1]))})
        }
    } else {
        //nested rule
        let rule_num = rule.parse::<i32>().unwrap();
        return parse_rule(rule_map, rule_map.get(&rule_num).unwrap());
    }
}