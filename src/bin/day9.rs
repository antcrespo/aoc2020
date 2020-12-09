
use std::fs;
use std::collections::HashSet;

fn main() {

    let input = fs::read_to_string("./src/input/day9.txt")
        .expect("Something went wrong reading the file");
    let mut numbers = Vec::new();
    for line in input.split("\r\n") {
        // println!("{}", line);
        let value = line.parse::<i64>().unwrap();
        numbers.push(value);
    }
    let window = 25;
    let silver = find_weak_number(&numbers, window);
    println!("Silver: {}", silver);
    let gold = find_contiguous_sum(&numbers, silver);
    println!("Gold: {}", gold)
}

fn find_weak_number(numbers: &[i64], window: usize) -> i64 {
    for (i, x) in (&numbers[window..]).iter().enumerate() {
        // println!("{} {}", i, x);
        if !check_valid_sum(&numbers[(i)..(i+window)], *x) {
            return *x;
        }
    }
    panic!("No weak number found");
}

fn check_valid_sum(window: &[i64], sum: i64) -> bool {
    let mut set = HashSet::new();
    for number in window {
        // println!("\t{}", number);
        if set.contains(&(sum-number)) {
            return true;
        }
        set.insert(number);
    }
    return false;
}

fn find_contiguous_sum(numbers: &[i64], goal: i64) -> i64 {
    let mut sum = numbers[0];
    let mut win_start = 0;
    let mut win_end = 0;
    'outer: while win_end < numbers.len() {
        win_end += 1;
        sum += numbers[win_end];
        if sum == goal {
            break 'outer;
        } if sum < goal {
            continue;
        } else {
            while sum > goal {
                sum -= numbers[win_start];
                win_start += 1;
                if sum == goal {
                    break 'outer;
                }
            }
        }
    }
    let mut min = numbers[win_start];
    let mut max = numbers[win_end];
    for i in win_start..=win_end {
        if numbers[i] < min {
            min = numbers[i];
        }
        if numbers[i] > max {
            max = numbers[i];
        }
    }
    return min + max;
}