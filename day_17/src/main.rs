use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

const FILE_LOC: &'static str = "data/test.txt";
fn main() {
    problem_one();
}

#[derive(Clone, Debug)]
struct Pathtree {
    position: (i64, i64),
    paths: Vec<Pathtree>,
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let map = parse_problem(lines);

    let location = Pathtree {
        position: (0, 0),
        paths: Vec::new(),
    };

    // let mut tracked_locations = HashSet::new();

    let mut locations = vec![location];

    // while locations.len() > 0 {
    let mut removed_so_far = 0;

    for (counter, loc) in locations.iter_mut().enumerate() {
        for move_speed in -3..=3 {
            for move_speed_xy in -3..=3 {
                let next_position = (loc.position.0 + move_speed, loc.position.1 + move_speed_xy);
                if next_position.0 < 0 || next_position.0 as usize >= map.len() {
                    continue;
                }
                if next_position.1 < 0 || next_position.1 as usize >= map[0].len() {
                    continue;
                }
                loc.paths.push(Pathtree {
                    position: next_position,
                    paths: Vec::new(),
                });
            }
        }
        // println!("{:?}", next_positions);
        // let is_new_loc = tracked_locations.insert(loc.clone());

        // if !is_new_loc {
        //     locations.remove(counter - removed_so_far);
        //     removed_so_far += 1;
        //     continue;
        // }
        // }
    }

    println!("{:?}", &locations);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    direction: Direction,
    same_direction_in_a_row: i64,
    position: (i64, i64),
}
impl Location {
    fn pos_to_usize(&self) -> (usize, usize) {
        (self.position.0 as usize, self.position.1 as usize)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn next_step(&self, current_step: (i64, i64), move_speed: i64) -> (i64, i64) {
        match self {
            Direction::Left => (current_step.0, current_step.1 - move_speed),
            Direction::Right => (current_step.0, current_step.1 + move_speed),
            Direction::Up => (current_step.0 - move_speed, current_step.1),
            Direction::Down => (current_step.0 + move_speed, current_step.1),
        }
    }
}

fn parse_problem(lines: Lines<BufReader<File>>) -> Vec<Vec<u64>> {
    let mut rows = Vec::new();
    for line in lines {
        let env_vec = line
            .as_ref()
            .unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        rows.push(env_vec);
    }

    return rows;
}
