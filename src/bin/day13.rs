use std::fs;


fn main() {

    let input = fs::read_to_string("./src/input/day13.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.split("\r\n").collect();
    let start: i32 = lines[0].parse().unwrap();
    let buses: Vec<&str> = lines[1].split(",").collect();
    let mut min = 9999999;
    let mut min_bus = 0;
    let mut remainder = 0;
    let mut reqs = Vec::new();
    let mut rems = Vec::new();
    for bus in buses {
        if bus == "x" {
            remainder += 1;
            continue;
        }
        let bus_num: i32 = bus.parse().unwrap();
        let wait_time = bus_num - (start % bus_num);
        if wait_time < min {
            min = wait_time;
            min_bus = bus_num;
        }
        reqs.push(bus_num as i64);
        let rem = (bus_num - (remainder%bus_num)) % bus_num;
        rems.push(rem as i64);
        remainder += 1;
    }
    println!("Silver {}", min_bus * min);
    let ans = chinese_remainder(rems.as_slice(), reqs.as_slice());
    println!("Gold: {}", ans);
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;
    
    while r != 0 {
        let quotient = old_r / r;

        let temp_r = r;
        r = old_r - quotient * r;
        old_r = temp_r;

        let temp_s = s;
        s = old_s - quotient * s;
        old_s = temp_s;

        let temp_t = t;
        t = old_t - quotient * t;
        old_t = temp_t;
    }
    return (old_r, old_s, old_t);
}
 
fn mod_inv(x: i64, n: i64) -> i64 {
    let (g, x, _) = extended_gcd(x, n);
    if g != 1 {
        panic!("Values not coprime");
    }
    return (x % n + n) % n;
    
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> i64 {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus) * p
    }
 
    return sum % prod;
}