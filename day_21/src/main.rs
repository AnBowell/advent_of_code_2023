use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    vec,
};

const FILE_LOC: &'static str = "data/test.txt";

fn main() {
    // problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (env, start) = parse_problem(lines);

    // Add in the buffer of Gardens

    // env.insert(0, vec![Environment::Plot; env[0].len()]);
    // env.push(vec![Environment::Plot; env[0].len()]);

    // for row in env.iter_mut() {
    //     row.insert(0, Environment::Plot);
    //     row.push(Environment::Plot);
    //     println!("{:?}", row);
    // }

    let mut plots = vec![vec![(start.0 as i64, start.1 as i64)]];

    let mut cache: HashMap<(i64, i64), HashSet<(i64, i64)>> = HashMap::with_capacity(10000);

    for step_counter in 0..26501365 {
        let mut unique_positions_this_step = HashSet::with_capacity(1000);
        for plot in &plots[step_counter] {
            let new_positions = match cache.get(plot) {
                Some(x) => x.to_owned(),
                None => {
                    let new_positions = step_wrapping(&env, *plot);
                    cache.insert(*plot, new_positions.clone());
                    new_positions
                }
            };

            for pos in new_positions.into_iter() {
                unique_positions_this_step.insert(pos);
            }
        }

        plots.push(unique_positions_this_step.into_iter().collect());

        if step_counter == 1000 - 1 {
            println!("Problem two total 1000: {}", plots.last().unwrap().len())
        }

        if step_counter == 5000 - 1 {
            println!("Problem two total 5000: {}", plots.last().unwrap().len())
        }
    }

    println!("Problem two total: {}", plots.last().unwrap().len())
}

// need a recursive cache again I think. When you get to a boundary that you've seen before it just repeats..
fn step_wrapping(env: &Vec<Vec<Environment>>, current_pos: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut plots_to_return = HashSet::with_capacity(200);
    for row_counter in -1..=1 {
        for col_counter in -1..=1 {
            // We don't want diagonal movement, or the start pos back. We also don't want to index something we can't.
            if (row_counter as i64 - col_counter as i64).abs() != 1 {
                continue;
            }

            let row = current_pos.0 + row_counter;
            let col = current_pos.1 + col_counter;

            let row_index = if row >= env.len() as i64 {
                row % env.len() as i64
            } else if row < 0 {
                let mut rem = row.abs() % env.len() as i64;
                if rem == 0 {
                    rem += 1
                };
                env.len() as i64 - rem
            } else {
                row
            };
            let col_index = if col >= env[0].len() as i64 {
                col % env[0].len() as i64
            } else if col < 0 {
                let mut rem = col.abs() % env[0].len() as i64;
                if rem == 0 {
                    rem += 1
                };

                env[0].len() as i64 - rem
            } else {
                col
            };

            match env[row_index as usize][col_index as usize] {
                Environment::Plot => {
                    plots_to_return.insert((row, col));
                }
                Environment::Rock => (),
            }
        }
    }
    return plots_to_return;
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (env, start) = parse_problem(lines);

    let mut plots = vec![vec![start]];

    for step_counter in 0..64 {
        let mut unique_positions_this_step = HashSet::new();
        for plot in &plots[step_counter] {
            let new_positions = step(&env, (plot.0 as i64, plot.1 as i64));

            for pos in new_positions.into_iter() {
                unique_positions_this_step.insert(pos);
            }
        }
        plots.push(unique_positions_this_step.into_iter().collect())
    }

    println!("Problem one total: {}", plots.last().unwrap().len())
}

fn step(env: &Vec<Vec<Environment>>, current_pos: (i64, i64)) -> HashSet<(usize, usize)> {
    let mut plots_to_return = HashSet::with_capacity(200);
    for row_counter in -1..=1 {
        for col_counter in -1..=1 {
            // We don't want diagonal movement, or the start pos back. We also don't want to index something we can't.
            if current_pos.0 + row_counter < 0
                || current_pos.0 + row_counter >= env.len() as i64
                || current_pos.1 + col_counter < 0
                || current_pos.1 + col_counter >= env[0].len() as i64
                || (row_counter - col_counter).abs() != 1
            {
                continue;
            }

            let row = (current_pos.0 + row_counter) as usize;
            let col = (current_pos.1 + col_counter) as usize;

            match env[row][col] {
                Environment::Plot => {
                    plots_to_return.insert((row, col));
                }
                Environment::Rock => (),
            }
        }
    }
    return plots_to_return;
}

#[derive(Debug, Clone)]
enum Environment {
    Plot,
    Rock,
}

fn parse_problem(lines: Lines<BufReader<File>>) -> (Vec<Vec<Environment>>, (usize, usize)) {
    let mut output_vec = Vec::new();
    let mut starting_pos = (0, 0);
    for (row_counter, line) in lines.enumerate() {
        let mut this_row_vec = Vec::new();
        for (col_counter, character) in line.as_ref().unwrap().chars().enumerate() {
            match character {
                'S' => {
                    starting_pos = (row_counter, col_counter);
                    this_row_vec.push(Environment::Plot)
                }
                '.' => this_row_vec.push(Environment::Plot),
                '#' => this_row_vec.push(Environment::Rock),
                _ => panic!("Ahhh invalid char found"),
            }
        }
        output_vec.push(this_row_vec)
    }

    return (output_vec, starting_pos);
}
