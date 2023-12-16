use std::{
    collections::HashSet,
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

    let contraption = parse_problem(lines);

    let mut max_configuration = 0;
    let starting_locs = generate_possible_starting_coords(contraption.len(), contraption[0].len());

    for location in starting_locs {
        let mut tracked_locations = HashSet::new();

        let mut locations = vec![location];

        while locations.len() > 0 {
            let mut removed_so_far = 0;

            for (counter, loc) in locations.clone().iter().enumerate() {
                let is_new_loc = tracked_locations.insert(loc.clone());

                if !is_new_loc {
                    locations.remove(counter - removed_so_far);
                    removed_so_far += 1;
                    continue;
                }

                let next_position = loc.direction.next_step(loc.position);

                if next_position.0 < 0 || next_position.0 as usize >= contraption.len() {
                    locations.remove(counter - removed_so_far);
                    removed_so_far += 1;
                    continue;
                }
                if next_position.1 < 0 || next_position.1 as usize >= contraption[0].len() {
                    locations.remove(counter - removed_so_far);
                    removed_so_far += 1;
                    continue;
                }

                let mut next_directions = contraption[next_position.0 as usize]
                    [next_position.1 as usize]
                    .next_direction(&loc.direction)
                    .iter()
                    .map(|x| Location {
                        direction: x.to_owned(),
                        position: next_position,
                    })
                    .collect::<Vec<Location>>();

                locations.remove(counter - removed_so_far);
                removed_so_far += 1;
                locations.append(&mut next_directions);
            }
        }

        // The map contains all the unqiue positions and directions. We just want unique tiles.
        max_configuration = max_configuration.max(
            tracked_locations
                .iter()
                .map(|x| x.position)
                .collect::<HashSet<(i64, i64)>>()
                .len()
                - 1, // Because we're starting at -1 on the right and we don't want that.
        );
    }
    println!("Problem two max config: {}", max_configuration);
}

fn generate_possible_starting_coords(row_len: usize, col_len: usize) -> Vec<Location> {
    (0..col_len)
        .map(|x| Location {
            direction: Direction::Right,
            position: (x as i64, -1),
        })
        .chain((0..col_len).map(|x| Location {
            direction: Direction::Left,
            position: (x as i64, row_len as i64),
        }))
        .chain((0..row_len).map(|x| Location {
            direction: Direction::Down,
            position: (-1, x as i64),
        }))
        .chain((0..row_len).map(|x| Location {
            direction: Direction::Up,
            position: (col_len as i64, x as i64),
        }))
        .collect()
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let contraption = parse_problem(lines);

    let location = Location {
        direction: Direction::Right,
        position: (0, -1),
    };

    let mut tracked_locations = HashSet::new();

    let mut locations = vec![location];

    while locations.len() > 0 {
        let mut removed_so_far = 0;

        for (counter, loc) in locations.clone().iter().enumerate() {
            let is_new_loc = tracked_locations.insert(loc.clone());

            if !is_new_loc {
                locations.remove(counter - removed_so_far);
                removed_so_far += 1;
                continue;
            }

            let next_position = loc.direction.next_step(loc.position);

            if next_position.0 < 0 || next_position.0 as usize >= contraption.len() {
                locations.remove(counter - removed_so_far);
                removed_so_far += 1;
                continue;
            }
            if next_position.1 < 0 || next_position.1 as usize >= contraption[0].len() {
                locations.remove(counter - removed_so_far);
                removed_so_far += 1;
                continue;
            }

            let mut next_directions = contraption[next_position.0 as usize]
                [next_position.1 as usize]
                .next_direction(&loc.direction)
                .iter()
                .map(|x| Location {
                    direction: x.to_owned(),
                    position: next_position,
                })
                .collect::<Vec<Location>>();

            locations.remove(counter - removed_so_far);
            removed_so_far += 1;
            locations.append(&mut next_directions);
        }
    }

    // The map contains all the positions and directions. We just want unique tiles.
    println!(
        "Problem one total: {}",
        tracked_locations
            .iter()
            .map(|x| x.position)
            .collect::<HashSet<(i64, i64)>>()
            .len()
            - 1 // Because we're starting at -1 on the right!
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    direction: Direction,
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
    fn next_step(&self, current_step: (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Left => (current_step.0, current_step.1 - 1),
            Direction::Right => (current_step.0, current_step.1 + 1),
            Direction::Up => (current_step.0 - 1, current_step.1),
            Direction::Down => (current_step.0 + 1, current_step.1),
        }
    }
}

#[derive(Debug, Clone)]
enum Component {
    Empty,
    ForwardMirror,
    BackMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Component {
    fn from_char(character: &char) -> Self {
        match character {
            '.' => Self::Empty,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackMirror,
            _ => panic!("Ahhh"),
        }
    }
    fn next_direction(&self, current_direction: &Direction) -> Vec<Direction> {
        match self {
            Component::Empty => vec![current_direction.to_owned()],
            Component::ForwardMirror => match current_direction {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            Component::BackMirror => match current_direction {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            Component::VerticalSplitter => match current_direction {
                Direction::Up | Direction::Down => vec![current_direction.to_owned()],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
            Component::HorizontalSplitter => match current_direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![current_direction.to_owned()],
            },
        }
    }
}
fn parse_problem(lines: Lines<BufReader<File>>) -> Vec<Vec<Component>> {
    let mut rows = Vec::new();
    for line in lines {
        let env_vec = line
            .as_ref()
            .unwrap()
            .chars()
            .map(|x| Component::from_char(&x))
            .collect::<Vec<Component>>();
        rows.push(env_vec);
    }

    return rows;
}
