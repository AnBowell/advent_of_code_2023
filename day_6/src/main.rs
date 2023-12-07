use std::{fs::File, io::Read};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let mut file = File::open(FILE_LOC).unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).unwrap();

    let mut lines = input_string.lines();

    let time = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    // Could just go through half and

    let halfway_rem = time % 2;

    let total = (1..(time / 2) + halfway_rem)
        .map(|t| t * (time - t))
        .filter(|y| y > &distance)
        .count()
        * 2
        + (1 - halfway_rem as usize);

    println!("Problem two total: {}", total);
}

fn problem_one() {
    let mut file = File::open(FILE_LOC).unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).unwrap();

    let mut lines = input_string.lines();

    let times = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let distance_records = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let mut total = 1;
    for (time, distance_record) in times.zip(distance_records) {
        let halfway_rem = time % 2;

        total *= (1..(time / 2) + halfway_rem)
            .map(|t| t * (time - t))
            .filter(|y| y > &distance_record)
            .count()
            * 2
            + (1 - halfway_rem as usize);
    }
    println!("Problem one total: {}", total);
}
