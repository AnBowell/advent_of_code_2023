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

    let mut coords = vec![(0_i64, 0_i64)];

    
    // Get outer line total. Can't make a vec lazily this time. Will get diff between vertices.
    let mut outer_line_total = 0;

    for line in lines {
        let mut direction_speed_colour = line.as_ref().unwrap().split_whitespace();

        let hex_speed_and_direction = direction_speed_colour
            .nth(2)
            .unwrap()
            .replace("(", "")
            .replace(")", "")
            .replace("#", "");

        let direction = direction_from_num(
            hex_speed_and_direction
                [hex_speed_and_direction.len() - 1..hex_speed_and_direction.len()]
                .parse::<u64>()
                .unwrap(),
        );

        let speed = hex_speed_and_direction[..hex_speed_and_direction.len() - 1]
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (pos, x)| {
                acc + i64::from_str_radix(&x.to_string(), 16).unwrap() * 16_i64.pow(pos as u32)
            });

        let last_coord = coords.last().unwrap();

        let next_coords = coords_from_direction_and_speed(&direction, last_coord, speed);

        outer_line_total +=
            (next_coords.0 - last_coord.0).abs() + (next_coords.1 - last_coord.1).abs();

        coords.push(next_coords);
    }

    let (x, y): (Vec<i64>, Vec<i64>) = coords
        .iter()
        .rev()
        .map(|x| ((x.0) as i64, (x.1) as i64))
        .unzip();

    // Shoelace polygon area technique!
    let area = polygon_area(&x, &y, x.len() as i64);

    println!("Problem two area: {:?}", area);

    // Then use Pick's theorem to get the amount of points inside!
    let i = area as f64 - (outer_line_total as f64 / 2.0) + 1.0;

    // Add them together.
    println!("Problem two total: {}", i + outer_line_total as f64);
}

fn direction_from_num(num: u64) -> char {
    match num {
        0 => 'R',
        1 => 'D',
        2 => 'L',
        3 => 'U',
        _ => panic!("ahhhh, invalid num"),
    }
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut coords = vec![(0_i64, 0_i64)];

    let mut outer_line_total = 0;

    for line in lines {
        let mut direction_speed_colour = line.as_ref().unwrap().split_whitespace();

        let direction = direction_speed_colour
            .next()
            .unwrap()
            .parse::<char>()
            .unwrap();

        let speed = direction_speed_colour
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        outer_line_total += speed;

        let _colour = direction_speed_colour
            .next()
            .unwrap()
            .replace("(", "")
            .replace(")", "");

        let mut next_coords =
            coords_from_direction_and_speed_vec(&direction, coords.last().unwrap(), speed);

        coords.append(&mut next_coords);
    }

    let (x, y): (Vec<i64>, Vec<i64>) = coords
        .iter()
        .rev()
        .map(|x| ((x.0) as i64, (x.1) as i64))
        .unzip();

    // Shoelace polygon area technique!
    let area = polygon_area(&x, &y, x.len() as i64);

    println!("Problem one area: {:?}", area);

    // Then use Pick's theorem to get the amount of points inside!
    let i = area as f64 - (outer_line_total as f64 / 2.0) + 1.0;

    // Add them together.
    println!("Problem one total: {}", i + outer_line_total as f64);
}

fn polygon_area(x: &Vec<i64>, y: &Vec<i64>, len: i64) -> i64 {
    let mut area = 0;
    for i in 0..len as usize - 2 {
        area += (x[i] * y[i + 1]) - (y[i] * x[i + 1])
    }
    area += (x[len as usize - 1] * y[0]) - (y[len as usize - 1] * x[0]);

    area = area.abs() / 2;
    return area;
}

fn coords_from_direction_and_speed(
    direction: &char,
    current_coords: &(i64, i64),
    speed: i64,
) -> (i64, i64) {
    match direction {
        'R' => (current_coords.0 + speed, current_coords.1),
        'L' => (current_coords.0 - speed, current_coords.1),
        'D' => (current_coords.0, current_coords.1 - speed),
        'U' => (current_coords.0, current_coords.1 + speed),
        _ => panic!("Cannot be"),
    }
}

// The lazy way... should have just done it like above!
fn coords_from_direction_and_speed_vec(
    direction: &char,
    current_coords: &(i64, i64),
    speed: i64,
) -> Vec<(i64, i64)> {
    match direction {
        'L' => (current_coords.0 - speed..current_coords.0)
            .map(|x| (x, current_coords.1))
            .rev()
            .collect(),
        'R' => (current_coords.0 + 1..=current_coords.0 + speed)
            .map(|x| (x, current_coords.1))
            .collect(),
        'D' => (current_coords.1 - speed..current_coords.1)
            .map(|x| (current_coords.0, x))
            .rev()
            .collect(),
        'U' => (current_coords.1 + 1..=current_coords.1 + speed)
            .map(|x| (current_coords.0, x))
            .collect(),
        _ => panic!("Cannot be"),
    }
}
