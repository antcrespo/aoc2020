
use std::fs;
use regex::Regex;


const NESW: &'static [&'static str] = &["N", "E", "S", "W"];

struct Boat {
    pos: (i32, i32),
    direction: i32,
}

struct GoldBoat {
    pos: (i32, i32),
    waypoint: (i32, i32),
} 

fn main() {

    let input = fs::read_to_string("./src/input/day12.txt")
        .expect("Something went wrong reading the file");
    let directions_regex: Regex = Regex::new(r"([A-Z])(\d+)").unwrap();
    let mut boat: Boat = Boat {pos: (0,0), direction: 1};
    let mut boat_gold: GoldBoat = GoldBoat {pos: (0,0), waypoint: (10, 1)};
    for step in directions_regex.captures_iter(&input) {
        let direction = &step[1];
        let value = step[2].parse::<i32>().unwrap();
        boat = move_boat(boat, direction, value);
        boat_gold = move_boat_gold(boat_gold, direction, value);
    }
    println!("Silver: {}", boat.pos.0.abs() + boat.pos.1.abs());
    println!("Gold: {}", boat_gold.pos.0.abs() + boat_gold.pos.1.abs());
    
   
}

fn move_boat_gold(boat: GoldBoat, instruction: &str, value: i32) -> GoldBoat {
    match instruction {
        "N" => return GoldBoat {pos: boat.pos, waypoint: (boat.waypoint.0, boat.waypoint.1 + value)},
        "S" => return GoldBoat {pos: boat.pos, waypoint: (boat.waypoint.0, boat.waypoint.1 - value)},
        "E" => return GoldBoat {pos: boat.pos, waypoint: (boat.waypoint.0 + value, boat.waypoint.1)},
        "W" => return GoldBoat {pos: boat.pos, waypoint: (boat.waypoint.0 - value, boat.waypoint.1)},
        "R" => return GoldBoat {pos: boat.pos, waypoint: rotate_point(boat.waypoint, value)},
        "L" => return GoldBoat {pos: boat.pos, waypoint: rotate_point(boat.waypoint, 360 - value)},
        "F" => return GoldBoat {pos: (boat.pos.0 + value * boat.waypoint.0, boat.pos.1 + value * boat.waypoint.1), waypoint: boat.waypoint},
        _ => panic!("Unrecognized step {}", instruction),
    }
}

fn move_boat(boat: Boat, instruction: &str, value: i32) -> Boat {
    let facing = NESW[boat.direction as usize];
    match instruction {
        "N" => return Boat {pos: (boat.pos.0, boat.pos.1 + value), direction: boat.direction},
        "S" => return Boat {pos: (boat.pos.0, boat.pos.1 - value), direction: boat.direction},
        "E" => return Boat {pos: (boat.pos.0 + value, boat.pos.1), direction: boat.direction},
        "W" => return Boat {pos: (boat.pos.0 - value, boat.pos.1), direction: boat.direction},
        "R" => return Boat {pos: boat.pos, direction: (boat.direction + (value/90)) % 4},
        "L" => return Boat {pos: boat.pos, direction: (boat.direction + ((360 - value)/90)) % 4},
        "F" => return move_boat(boat, facing, value),
        _ => panic!("Unrecognized step {}", instruction),
    }
}

fn rotate_point(mut point: (i32, i32), mut degrees: i32) -> (i32, i32) {
    while degrees > 0 {
        let n = if point.1 > 0 {point.1} else {0};
        let s = if point.1 < 0 {-1 * point.1} else {0};
        let e = if point.0 > 0 {point.0} else {0};
        let w = if point.0 < 0 {-1 * point.0} else {0};
        point = (n-s, w-e);
        degrees -= 90;
    }
    return point;
}
