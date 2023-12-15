use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

const FILE_LOC: &'static str = "data/input.txt";
fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut environment_vec = parse_problem(lines);

    // let mut total_sum = 0;

    let directions = vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut seen_so_far = HashMap::new();
    let cycle = directions.iter().cycle();

    let mut continue_running = None;
    let mut continue_running_counter = 0;

    for (dir_no, direction) in cycle.enumerate() {
        for (which_column, column) in environment_vec.iter_mut().enumerate() {
            let mut last_stopping_coord = -1;

            // Silly!!
            if matches!(direction, Direction::East) || matches!(direction, Direction::South) {
                column.reverse();
            }
            for (column_pos, env) in column.clone().iter().enumerate() {
                match env {
                    Environment::RoundedRock => {
                        column[(last_stopping_coord + 1) as usize] = env.clone();

                        if (last_stopping_coord + 1) != column_pos as i64 {
                            column[column_pos] = Environment::Empty;
                        }

                        last_stopping_coord += 1;
                    }
                    Environment::CubeRock => last_stopping_coord = column_pos as i64,
                    Environment::Empty => continue,
                }
            }
            if matches!(direction, Direction::East) || matches!(direction, Direction::South) {
                column.reverse();
            }
        }
        // Change the orientation each time.
        environment_vec = transpose(environment_vec);

        // Messy here. Didn't want to keep track of a history, so just step forward some amount...
        if continue_running.is_some() {
            if continue_running.unwrap() == continue_running_counter {
                let mut total_load_this_cycle = 0;
                for column in &environment_vec {
                    total_load_this_cycle += column
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| matches!(x, Environment::RoundedRock))
                        .map(|(count, _)| column.len() - count)
                        .sum::<usize>();
                }
                println!("Problem two answer: {}", total_load_this_cycle);
                break;
            }
            continue_running_counter += 1;
        }

        if (dir_no + 1) % 4 == 0 {
            let mut total_load_this_cycle = 0;
            for column in &environment_vec {
                total_load_this_cycle += column
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| matches!(x, Environment::RoundedRock))
                    .map(|(count, _)| column.len() - count)
                    .sum::<usize>();
            }

            let seen_before = seen_so_far.insert(total_load_this_cycle, (dir_no + 1) / 4);

            // Messy here. Didn't want to keep track of a history, so just step forward some amount...
            if seen_before.is_some() && continue_running.is_none() {
                let cycle_len = ((dir_no + 1) / 4) - seen_before.unwrap();

                let x = (1_000_000_000 - seen_before.unwrap()) / cycle_len;

                let y = seen_before.unwrap() + x * cycle_len;

                let diff = 1_000_000_000 - y;

                continue_running = Some((diff * 4) - 1);
            }
        }
    }
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut environment_vec = parse_problem(lines);

    let mut total_sum = 0;
    for (which_column, column) in environment_vec.clone().iter().enumerate() {
        let mut last_stopping_coord: i64 = -1;
        for (column_pos, env) in column.iter().enumerate() {
            match env {
                Environment::RoundedRock => {
                    environment_vec[which_column][(last_stopping_coord + 1) as usize] = env.clone();

                    if (last_stopping_coord + 1) != column_pos as i64 {
                        environment_vec[which_column][column_pos] = Environment::Empty;
                    }
                    last_stopping_coord += 1;

                    total_sum += column.len() as i64 - last_stopping_coord;
                }
                Environment::CubeRock => last_stopping_coord = column_pos as i64,
                Environment::Empty => continue,
            }
        }
    }

    println!("Problem one: {}", total_sum);
}

fn parse_problem(lines: Lines<BufReader<File>>) -> Vec<Vec<Environment>> {
    let mut rows = Vec::new();
    for line in lines {
        let env_vec = line
            .as_ref()
            .unwrap()
            .chars()
            .map(|x| Environment::env_from_char(&x))
            .collect::<Vec<Environment>>();
        rows.push(env_vec);
    }

    return transpose(rows);
}

// enum IterRev<'a> {
//     NonRev(Enumerate<Iter<'a, Environment>>>),
//     Rev(Enumerate<std::iter::Rev<Iter<'a, Environment>>>),
// }

enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone)]
enum Environment {
    RoundedRock,
    CubeRock,
    Empty,
}

impl Environment {
    fn env_from_char(character: &char) -> Self {
        match character {
            'O' => Self::RoundedRock,
            '#' => Self::CubeRock,
            '.' => Self::Empty,
            _ => panic!("Ahhh invalid."),
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
