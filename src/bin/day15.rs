use std::collections::HashMap;

fn main() {

    let input = "7,14,0,17,11,1,2";
    let starters = input.split(",");
    
    let mut round = 0;
    let mut last_said = HashMap::new();
    let mut last_word = 0;
    let mut difference = 0;
    for starter in starters {
        let num: i64 = starter.parse().unwrap();
        let last_round = last_said.get(&num).unwrap_or(&round);
        difference = round - last_round;
        last_said.insert(num, round);
        round += 1;
        last_word = num;
    }
    while round < 2020 {
        let num = difference;
        let last_round = last_said.get(&num).unwrap_or(&round);
        difference = round - last_round;
        last_said.insert(num, round);
        round += 1;
        last_word = num;
    }
    println!("Silver: {}", last_word);
    while round < 30000000 {
        let num = difference;
        let last_round = last_said.get(&num).unwrap_or(&round);
        difference = round - last_round;
        last_said.insert(num, round);
        round += 1;
        last_word = num;
    }
    println!("Gold: {}", last_word);
}
