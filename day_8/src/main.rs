use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    hash,
    io::{self, BufRead, BufReader, Lines},
};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let instructions = lines.next().unwrap().unwrap();

    lines.next();

    let hashmap = load_in_nodes_to_hashmap(&mut lines);

    let current_nodes = hashmap
        .keys()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .collect::<Vec<&String>>();

    let mut total_steps = Vec::new();

    for mut current_node in current_nodes.into_iter() {
        {
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

                if current_node.chars().last().unwrap() == 'Z' {
                    total_steps.push(amount_of_steps);
                    // println!("Total steps: {:?}", total_steps);
                    break;
                }
            }
        }
    }

    total_steps.sort();

    println!("Total amount of steps: {:?}", total_steps);

    let mut current_lcm = (total_steps[0] * total_steps[1]) / gcd(total_steps[0], total_steps[1]);

    for counter in 1..total_steps.len() {
        current_lcm = (total_steps[counter] * current_lcm) / gcd(total_steps[counter], current_lcm);
    }
    println!("Current lcm: {}", current_lcm);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut r;
    while a % b > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    return b;
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let instructions = lines.next().unwrap().unwrap();

    lines.next();

    let hashmap = load_in_nodes_to_hashmap(&mut lines);

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
