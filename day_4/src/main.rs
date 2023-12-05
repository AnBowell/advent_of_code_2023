/// Pleased with this solution! Clean, concise, and fast!
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut copy_tracker = HashMap::with_capacity(200);

    for line in lines {
        let line_str: &String = line.as_ref().unwrap();
        let mut line_split = line_str.split(":");

        let card_no = line_split
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let number_of_cards = copy_tracker
            .entry(card_no)
            .and_modify(|x| *x += 1) // Add one for the original!
            .or_insert(1);

        let mut numbers = line_split.next().unwrap().split("|");

        let winning_numbers: HashSet<i64> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let my_numbers: Vec<i64> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let line_matches = my_numbers
            .iter()
            .filter(|x| winning_numbers.contains(x))
            .count();

        for _card_copy_no in 1..=*number_of_cards {
            for counter in 1..=line_matches {
                copy_tracker
                    .entry(card_no + counter as i64)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    let total = copy_tracker.iter().fold(0, |acc, (_, count)| acc + count);

    println!("Problem two total: {}", total);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total = 0;

    for line in lines {
        let line_str: &String = line.as_ref().unwrap();
        let mut line_split = line_str.split(":");

        let _card_no = line_split.next().unwrap();

        let mut numbers = line_split.next().unwrap().split("|");

        let winning_numbers: HashSet<i64> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let my_numbers: Vec<i64> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let line_matches = my_numbers
            .iter()
            .filter(|x| winning_numbers.contains(x))
            .count();

        let line_score = (2 as i64).pow(line_matches as u32) / 2;
        total += line_score;
    }

    println!("Problem one total: {}", total);
}
