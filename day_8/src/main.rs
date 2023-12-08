use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    hash,
    io::{self, BufRead, BufReader, Lines},
};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    // problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let instructions = lines.next().unwrap().unwrap();

    lines.next();

    let hashmap = load_in_nodes_to_hashmap(&mut lines);

    let mut current_nodes = hashmap
        .keys()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .collect::<Vec<&String>>();

    let mut amount_of_steps = 0;

    for left_or_right in instructions.chars().cycle() {
        let mut end_in_things_but_z_count = 0;
        for current_node in current_nodes.iter_mut() {
            // println!("Current node: {}", current_node);
            if left_or_right == 'L' {
                *current_node = &hashmap.get(current_node.as_str()).unwrap().0
            } else if left_or_right == 'R' {
                *current_node = &hashmap.get(current_node.as_str()).unwrap().1
            } else {
                panic!("Ahhhh how")
            }

            if current_node.chars().last().unwrap() != 'Z' {
                end_in_things_but_z_count += 1;
            }
        }
        // println!("Current nodes: {:?}", current_nodes);
        amount_of_steps += 1;
        if end_in_things_but_z_count == 0 {
            break;
        }
    }

    println!("Problem two amount of steps: {}", amount_of_steps)
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let instructions = lines.next().unwrap().unwrap();

    lines.next();

    let hashmap = load_in_nodes_to_hashmap(&mut lines);
    println!("Hashmap: {:?}", hashmap);
    let mut current_node = "AAA";
    let mut amount_of_steps = 0;

    for left_or_right in instructions.chars().cycle() {
        // println!("Current node: {}", current_node);
        if left_or_right == 'L' {
            current_node = &hashmap.get(current_node).unwrap().0
        } else if left_or_right == 'R' {
            current_node = &hashmap.get(current_node).unwrap().1
        } else {
            panic!("Ahhhh how")
        }

        amount_of_steps += 1;

        if current_node == "ZZZ" {
            break;
        }
    }

    println!("Problem one amount of steps: {}", amount_of_steps)
}

fn load_in_nodes_to_hashmap(
    lines: &mut Lines<BufReader<File>>,
) -> HashMap<String, (String, String)> {
    let mut hashmap = HashMap::new();

    for line in lines {
        let line_ref = line.as_ref().unwrap();

        let mut split_line = line_ref.split(" = ");

        let value = split_line.next().unwrap().to_string();

        let mut nodes = split_line.next().unwrap().split(", ");

        let left = nodes.next().unwrap().replace("(", "");
        let right = nodes.next().unwrap().replace(")", "");

        hashmap.insert(value, (left, right));
    }

    return hashmap;
}
