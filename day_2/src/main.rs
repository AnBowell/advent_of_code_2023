use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};
pub const FILE_LOC: &'static str = "data/test.txt";

fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut result = 0;

    for line in lines {
        let line = line.as_ref().unwrap();

        let mut game_no_and_games = line.split(":");

        let _game_no = game_no_and_games
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let games = game_no_and_games.next().unwrap().split(";");

        let mut current_number = 0;
        let mut mapping = HashMap::new();
        for game in games {
            for number_or_colour in game.split_whitespace() {
                if number_or_colour.len() <= 2 {
                    current_number = number_or_colour.parse::<i64>().unwrap();
                } else {
                    let colour = number_or_colour.trim_end_matches(",");

                    let current_max = mapping.entry(colour).or_insert(current_number);
                    if &current_number > current_max {
                        mapping.insert(colour, current_number);
                    }
                }
            }
        }
        let this_game_total = mapping.iter().fold(1, |acc, (_, nums)| acc * nums);
        result += this_game_total;
    }

    println!("Result: {}", result);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut mapping = HashMap::new();

    mapping.insert("red", 12);
    mapping.insert("green", 13);
    mapping.insert("blue", 14);

    let mut result = 0;

    for line in lines {
        let line = line.as_ref().unwrap();

        let mut game_no_and_games = line.split(":");

        let game_no = game_no_and_games
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        result += game_no;

        let games = game_no_and_games.next().unwrap().split(";");

        let mut current_number = 0;
        'outer: for game in games {
            for number_or_colour in game.split_whitespace() {
                if number_or_colour.len() <= 2 {
                    current_number = number_or_colour.parse::<i64>().unwrap();
                } else {
                    if &current_number
                        > mapping.get(number_or_colour.trim_end_matches(",")).unwrap()
                    {
                        result -= game_no;
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("Result: {}", result);
}
