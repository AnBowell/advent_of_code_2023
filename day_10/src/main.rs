use core::panic;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

use geo::{coord, point, Contains, Polygon};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two();
}
//336  wrong
fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (map, start_coords) = parse_pipe_map(lines);

    let possible_paths = find_possible_starting_directions(start_coords, &map);

    let possible_paths: Vec<Direction> = possible_paths
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    assert_eq!(possible_paths.len(), 2); // Should be two paths.

    let (_, mut traversed_one, mut traversed_two) =
        traverse_path(possible_paths[0], possible_paths[1], start_coords, &map);

    traversed_two.pop();
    traversed_two.reverse();
    traversed_one.append(&mut traversed_two);

    let polygon = Polygon::new(
        geo::LineString(
            traversed_one
                .iter()
                .map(|(y, x)| coord! { x: *x  as i64, y: *y as i64 })
                .collect(),
        ),
        vec![],
    );

    let mut number_contained = 0;
    for row_counter in 0..map.len() {
        for column_counter in 0..map[0].len() {
            let point = point! {x: column_counter as i64, y: row_counter as i64};

            if polygon.contains(&point) {
                number_contained += 1;
            }
        }
    }
    println!("Problem two: {}", number_contained);
}
fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (map, start_coords) = parse_pipe_map(lines);

    let possible_paths = find_possible_starting_directions(start_coords, &map);

    let possible_paths: Vec<Direction> = possible_paths
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    assert_eq!(possible_paths.len(), 2); // Should be two paths.

    let (steps, _, _) = traverse_path(possible_paths[0], possible_paths[1], start_coords, &map);

    println!("Problem one: {}", steps);
}

fn traverse_path(
    mut direction_one: Direction,
    mut direction_two: Direction,
    starting_coords: (usize, usize),
    map: &Vec<Vec<Pipes>>,
) -> (usize, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut path_one_coords = starting_coords;
    let mut path_two_coords = starting_coords;

    let mut step_counter = 0;

    let mut traversed_pipes_one = Vec::new();

    let mut traversed_pipes_two = Vec::new();
    while (path_one_coords != path_two_coords) || (path_one_coords == starting_coords) {
        path_one_coords = direction_one.next_step(path_one_coords);
        direction_one = map[path_one_coords.0][path_one_coords.1].next_direction(&direction_one);

        path_two_coords = direction_two.next_step(path_two_coords);
        direction_two = map[path_two_coords.0][path_two_coords.1].next_direction(&direction_two);

        step_counter += 1;

        traversed_pipes_one.push(path_one_coords);
        traversed_pipes_two.push(path_two_coords);
    }

    return (step_counter, traversed_pipes_one, traversed_pipes_two);
}

#[derive(Debug)]
enum Pipes {
    Vertical,
    Horizontal,
    NorthLEast,
    NorthJWest,
    South7West,
    SouthFEast,
    Ground,
    Start,
}

impl Pipes {
    fn from_char(character: &char) -> Self {
        match character {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthLEast,
            'J' => Self::NorthJWest,
            '7' => Self::South7West,
            'F' => Self::SouthFEast,
            '.' => Self::Ground,
            'S' => Self::Start,

            _ => panic!("Cannot exist!"),
        }
    }
    fn next_direction(&self, current_direction: &Direction) -> Direction {
        match self {
            Pipes::Vertical => match current_direction {
                Direction::Up | Direction::Down => current_direction.clone(),
                _ => panic!("Can't move left/right in a vertical pipe"),
            }, // Up one, down one.
            Pipes::Horizontal => match current_direction {
                Direction::Left | Direction::Right => current_direction.clone(),
                _ => panic!("Can't move up/down in a horizontal pipe"),
            }, // Right one, left one.
            Pipes::NorthLEast => match current_direction {
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
                _ => panic!("Cannot move this way"),
            }, // Up one, right one.
            Pipes::NorthJWest => match current_direction {
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                _ => panic!("Cannot do"),
            }, // Up one, left one.
            Pipes::South7West => match current_direction {
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                _ => panic!("canny"),
            }, // Down one, left one.
            Pipes::SouthFEast => match current_direction {
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Right,
                _ => panic!("ahh"),
            }, // Down one, right one.
            Pipes::Ground => match current_direction {
                _ => panic!("ahh"),
            }, // Stop.
            Pipes::Start => panic!("Ahh, infinite loop"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn next_step(&self, current_step: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Left => (current_step.0, current_step.1 - 1),
            Direction::Right => (current_step.0, current_step.1 + 1),
            Direction::Up => (current_step.0 - 1, current_step.1),
            Direction::Down => (current_step.0 + 1, current_step.1),
        }
    }
}

fn find_possible_starting_directions(
    start: (usize, usize),
    map: &Vec<Vec<Pipes>>,
) -> Vec<Option<Direction>> {
    let right_path = if start.1 + 1 < map[0].len() {
        match map[start.0][start.1 + 1] {
            Pipes::Horizontal | Pipes::NorthJWest | Pipes::South7West => Some(Direction::Right),
            _ => None,
        }
    } else {
        None
    };

    let left_path = if start.1 > 0 {
        match map[start.0][start.1 - 1] {
            Pipes::Horizontal | Pipes::NorthLEast | Pipes::SouthFEast => Some(Direction::Left),
            _ => None,
        }
    } else {
        None
    };

    let up_path = if start.0 > 0 {
        match map[start.0 - 1][start.1] {
            Pipes::Vertical | Pipes::South7West | Pipes::SouthFEast => Some(Direction::Up),
            _ => None,
        }
    } else {
        None
    };

    let down_path = if start.0 + 1 < map.len() {
        match map[start.0 + 1][start.1] {
            Pipes::Vertical | Pipes::NorthLEast | Pipes::NorthJWest => Some(Direction::Down),
            _ => None,
        }
    } else {
        None
    };

    return vec![right_path, left_path, up_path, down_path];
}

fn parse_pipe_map(lines: Lines<BufReader<File>>) -> (Vec<Vec<Pipes>>, (usize, usize)) {
    let mut output_vec = Vec::with_capacity(200);

    let mut start_pos = (0, 0);

    for (line_counter, line) in lines.enumerate() {
        let mut line_vec = Vec::with_capacity(200);

        for (col_counter, character) in line.as_ref().unwrap().chars().enumerate() {
            let pipe = Pipes::from_char(&character);

            if matches!(pipe, Pipes::Start) {
                start_pos = (line_counter, col_counter);
            };

            line_vec.push(pipe)
        }

        output_vec.push(line_vec);
    }
    return (output_vec, start_pos);
}
