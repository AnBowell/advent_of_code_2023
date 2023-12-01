use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut mapping: HashMap<&str, char> = HashMap::new();
    mapping.insert("one", '1');
    mapping.insert("two", '2');
    mapping.insert("three", '3');
    mapping.insert("four", '4');
    mapping.insert("five", '5');
    mapping.insert("six", '6');
    mapping.insert("seven", '7');
    mapping.insert("eight", '8');
    mapping.insert("nine", '9');
    mapping.insert("1", '1');
    mapping.insert("2", '2');
    mapping.insert("3", '3');
    mapping.insert("4", '4');
    mapping.insert("5", '5');
    mapping.insert("6", '6');
    mapping.insert("7", '7');
    mapping.insert("8", '8');
    mapping.insert("9", '9');

    let mut total: i64 = 0;
    for line in lines {
        let line_ref = line.as_ref().unwrap();

        let mut first = usize::MAX;
        let mut first_len = 1;
        let mut last = usize::MIN;
        let mut last_len = 1;

        for key in mapping.keys() {
            match line_ref.find(key) {
                Some(x) => {
                    if x < first {
                        first = x;
                        first_len = key.len()
                    }
                }
                None => {}
            }
            match line_ref.rfind(key) {
                Some(x) => {
                    if x > last {
                        last = x;
                        last_len = key.len()
                    }
                }
                None => {}
            }
        }

        let first_key =
            String::from_utf8(line_ref.as_bytes()[first..(first + first_len)].to_vec()).unwrap();
        let last_key =
            String::from_utf8(line_ref.as_bytes()[last..(last + last_len)].to_vec()).unwrap();

        let mut result_string = String::new();

        result_string.push(mapping.get(first_key.as_str()).unwrap().to_owned());
        result_string.push(mapping.get(last_key.as_str()).unwrap().to_owned());

        total += result_string.parse::<i64>().unwrap();
    }
    println!("Total for problem two is: {}", total);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();

    let lines = io::BufReader::new(file).lines();

    let mut total: i64 = 0;
    for line in lines {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;
        for character in line.as_ref().unwrap().chars() {
            if character.is_numeric() {
                if first_digit.is_none() {
                    first_digit = Some(character)
                }
                last_digit = Some(character)
            }
        }

        let mut result_string = String::new();

        result_string.push(first_digit.unwrap());
        result_string.push(last_digit.unwrap());

        total += result_string.parse::<i64>().unwrap();
    }
    println!("Total for problem one is: {}", total);
}
