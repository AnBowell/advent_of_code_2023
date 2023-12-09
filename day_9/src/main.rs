use std::{
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

    let mut total_predicted = 0;
    for line in lines {
        let mut oasis_history = line
            .as_ref()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut predicted_value = oasis_history.first().unwrap().to_owned();

        let mut counter = 0;

        while oasis_history.iter().filter(|x| *x != &0).count() > 0 {
            oasis_history = diff(&oasis_history);

            if counter % 2 == 0 {
                predicted_value -= oasis_history.first().unwrap();
            } else {
                predicted_value += oasis_history.first().unwrap();
            }
            counter += 1;
        }

        total_predicted += predicted_value;
    }
    println!("Problem two: {}", total_predicted);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total_predicted = 0;
    for line in lines {
        let mut oasis_history = line
            .as_ref()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut predicted_value = oasis_history.last().unwrap().to_owned();

        while oasis_history.iter().filter(|x| *x != &0).count() > 0 {
            oasis_history = diff(&oasis_history);
            predicted_value += oasis_history.last().unwrap();
        }

        total_predicted += predicted_value;
    }
    println!("Problem one: {}", total_predicted);
}

fn diff(vec_to_diff: &Vec<i64>) -> Vec<i64> {
    let mut diff_vec = Vec::with_capacity(vec_to_diff.len() - 1);
    for counter in 0..(vec_to_diff.len() - 1) {
        diff_vec.push(vec_to_diff[counter + 1] - vec_to_diff[counter])
    }

    return diff_vec;
}
