use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("./src/input/day14.txt")
        .expect("Something went wrong reading the file");
    let mem_reg: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let lines = input.split("\r\n");
    let mut mask = "".to_string();
    let mut memory = HashMap::new();
    let mut gold_memory = HashMap::new();
    for line in lines {
        let start = &line[..4];
        if start == "mask" {
            mask = line[7..].to_string();
        } else {
            let caps = mem_reg.captures(line).unwrap();
            let address = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let val_string = convert_to_string(value);
            let val_masked = mask_string(&mask, val_string);
            let final_val = convert_from_string(val_masked);
            memory.insert(address, final_val);

            let address_string = convert_to_string(address);
            let masked_mem = mask_addresses(&mask, address_string);
            for mem in masked_mem {
                let adr_val = convert_from_string(mem);
                gold_memory.insert(adr_val, value);
            }
        }
    }
    let mut sum = 0;
    for (_, value) in &memory {
        sum += value;
    }
    println!("Silver: {}", sum);
    let mut gold_sum = 0;
    for (_, value) in &gold_memory {
        gold_sum += value;
    }
    println!("Gold: {}", gold_sum);
}

fn convert_to_string(val: u64) -> String {
    let mut mask: u64 = 34359738368;
    let mut res = "".to_string();
    for _ in 0..36 {
        let test = val & mask;
        if test == 0 {
            res.push('0');
        } else {
            res.push('1');
        }
        mask = mask >> 1;
    }
    return res;
}

fn convert_from_string(val: String) -> u64 {
    
    let mut res = 0;
    let mut mask: u64 = 34359738368;
    for bit in val.chars() {
        if bit == '1' {
            res += mask;
        }
        mask = mask >> 1;
    }
    return res;
}

fn mask_string(mask: &String, val: String) -> String {
    let mut res: String = "".to_string();
    for (mask_char, val_char) in mask.chars().zip(val.chars()) {
        if mask_char != 'X' {
            res.push(mask_char);
        } else {
            res.push(val_char);
        }
    }
    return res;
}

fn mask_addresses(mask: &String, val: String) -> Vec<String> {
    let start: String = "".to_string();
    let mut ans = Vec::new();
    ans.push(start);
    for (mask_char, val_char) in mask.chars().zip(val.chars()) {
        if mask_char == 'X' {
            let mut new_ans = Vec::new();
            for mut res in ans {
                let mut clone = res.clone();
                res.push('0');
                clone.push('1');
                new_ans.push(clone);
                new_ans.push(res);
            }
            ans = new_ans;
        } else if mask_char == '0' {
            for res in ans.iter_mut() {
                res.push(val_char);
            }
        } else {
            for res in ans.iter_mut() {
                res.push('1');
            }
        }
    }
    return ans;
}