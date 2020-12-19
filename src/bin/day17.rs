
use std::fs;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Node {
    active_neighbors: i32,
    active: bool,
}

fn main() {

    let input = fs::read_to_string("./src/input/day17.txt")
        .expect("Something went wrong reading the file");
    let lines = input.split("\r\n");
    let mut space: HashMap<(i32, i32, i32), Node> = HashMap::new();
    let mut space_4d: HashMap<(i32, i32, i32, i32), Node> = HashMap::new();
    let mut pos = (0,0,0);
    let mut pos_4d = (0,0,0, 0);
    for line in lines {
        pos.1 = 0;
        pos_4d.1 = 0;
        for symbol in line.chars() {
            let node = Node {active_neighbors: 0, active: symbol == '#'};
            space.insert(pos, node);
            space_4d.insert(pos_4d, node);
            pos.1 += 1;
            pos_4d.1 += 1;
        }
        pos.0 += 1;
        pos_4d.0 += 1;
    }

    for _ in 0..6 {
        let mut new_space: HashMap<(i32, i32, i32), Node> = HashMap::new();
        for (pos, node) in space.iter() {
            let neighbors = generate_neighbors(*pos);
            for neighbor in neighbors {
                if !new_space.contains_key(&neighbor) {
                    let old_neighbor = space.get(&neighbor).unwrap_or(&Node {active_neighbors: 0, active: false});
                    new_space.insert(neighbor, Node {active_neighbors: 0, active: old_neighbor.active});
                }
                let mut neighbor_node = new_space.get_mut(&neighbor).unwrap();
                if node.active {
                    neighbor_node.active_neighbors += 1;
                }
            }
        }
        for (_, node) in new_space.iter_mut() {
            if node.active {
                if !(node.active_neighbors == 2 || node.active_neighbors == 3) {
                    node.active = false;
                }
            } else if node.active_neighbors == 3 {
                node.active = true;
            }
        }
        space = new_space;

        //gold
        let mut new_space_4d: HashMap<(i32, i32, i32, i32), Node> = HashMap::new();
        for (pos, node) in space_4d.iter() {
            let neighbors = generate_neighbors_4d(*pos);
            for neighbor in neighbors {
                if !new_space_4d.contains_key(&neighbor) {
                    let old_neighbor = space_4d.get(&neighbor).unwrap_or(&Node {active_neighbors: 0, active: false});
                    new_space_4d.insert(neighbor, Node {active_neighbors: 0, active: old_neighbor.active});
                }
                let mut neighbor_node = new_space_4d.get_mut(&neighbor).unwrap();
                if node.active {
                    neighbor_node.active_neighbors += 1;
                }
            }
        }
        for (_, node) in new_space_4d.iter_mut() {
            if node.active {
                if !(node.active_neighbors == 2 || node.active_neighbors == 3) {
                    node.active = false;
                }
            } else if node.active_neighbors == 3 {
                node.active = true;
            }
        }
        space_4d = new_space_4d;
    }
    let mut silver = 0;
    for (_, node) in space {
        if node.active {
            silver += 1;
        }
    }
    println!("Silver: {}", silver);

    let mut gold = 0;
    for (_, node) in space_4d {
        if node.active {
            gold += 1;
        }
    }
    println!("Gold: {}", gold);
}


fn generate_neighbors(pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut res = Vec::new();
    for x in -1..=1 {
        for y in -1 ..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                res.push((pos.0+x, pos.1+y, pos.2+z));
            }
        }
    }
    return res;
}

fn generate_neighbors_4d(pos: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let mut res = Vec::new();
    for x in -1..=1 {
        for y in -1 ..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    res.push((pos.0+x, pos.1+y, pos.2+z, pos.3+w));
                }
            }
        }
    }
    return res;
}