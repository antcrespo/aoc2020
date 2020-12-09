
use std::fs;
use std::collections::HashSet;
use regex::Regex;

struct Instruction {
    operator: String,
    value: i32,
}

fn main() {

    let input = fs::read_to_string("./src/input/day8.txt")
        .expect("Something went wrong reading the file");
    let mut instructions = Vec::new();
    let instruction_regex: Regex = Regex::new(r"\b([a-z]{3}) \+?(-?\d+)\b").unwrap();
    for line in instruction_regex.captures_iter(&input) {
        let operator = line[1].to_string();
        let value = line[2].parse().unwrap();
        
        let instruction: Instruction = Instruction {operator: operator, value: value};
        instructions.push(instruction);
    }
    
    println!("Silver: {}", run_until_loop(&instructions, 9999999).value);
    
    for (i,_) in instructions.iter().enumerate() {
        let result = run_until_loop(&instructions, i);
        if result.operator == "end" {
            println!("Gold {}", result.value);
            break;
        }
    }
}

fn run_until_loop(instructions: &Vec<Instruction>, flip: usize) -> Instruction {
    let mut cur: usize = 0;
    let mut acc: i32 = 0;
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&cur) {
            return Instruction {operator: "loop".to_string(), value: acc};
        } else if cur >= instructions.len() {
            return Instruction {operator: "end".to_string(), value: acc};
        }
        let instruction = &instructions[cur];
        seen.insert(cur);
        // println!("{} {}", instruction.operator, instruction.value);
        if (instruction.operator == "jmp" && flip != cur) || (instruction.operator == "nop" && flip == cur) {
            if instruction.value < 0 {
                cur -= (-instruction.value) as usize;
            } else {
                cur += instruction.value as usize;
            }
            continue;
        } else if instruction.operator == "acc" {
            acc += instruction.value;
        }
        cur += 1;
    }
}