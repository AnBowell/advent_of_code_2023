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

    let galaxy_locs = parse_galaxies_locs_part_2(lines);

    let mut distance_sum = 0;
    for (galaxy_counter, galaxy) in galaxy_locs.iter().enumerate() {
        for other_galaxy in &galaxy_locs[galaxy_counter + 1..] {
            let diff = (other_galaxy.1 as i64 - galaxy.1 as i64).abs()
                + (other_galaxy.0 as i64 - galaxy.0 as i64).abs();

            distance_sum += diff;
        }
    }

    println!("Problem one: {}", distance_sum);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (_, galaxy_locs) = parse_image(lines);

    let mut distance_sum = 0;
    for (galaxy_counter, galaxy) in galaxy_locs.iter().enumerate() {
        for other_galaxy in &galaxy_locs[galaxy_counter + 1..] {
            let diff = (other_galaxy.1 as i64 - galaxy.1 as i64).abs()
                + (other_galaxy.0 as i64 - galaxy.0 as i64).abs();

            distance_sum += diff;
        }
    }

    println!("Problem two: {}", distance_sum);
}
#[derive(Debug, Clone)]
enum Pixel {
    Galaxy,
    Space,
}

impl Pixel {
    fn from_char(character: &char) -> Self {
        match character {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("Not allowed in input!"),
        }
    }
}

// Should have just done it this way for part 1 but I cba. Lesson learned.
fn parse_galaxies_locs_part_2(lines: Lines<BufReader<File>>) -> Vec<(usize, usize)> {
    let mut row_vec = Vec::with_capacity(140);
    let mut galaxy_vec = Vec::with_capacity(1000);

    let mut extra_space = 0;
    for (line_counter, line) in lines.enumerate() {
        let mut col_vec = Vec::with_capacity(140);

        let mut has_galaxy = false;
        for (char_counter, character) in line.as_ref().unwrap().chars().enumerate() {
            let pixel = Pixel::from_char(&character);
            if matches!(pixel, Pixel::Galaxy) {
                galaxy_vec.push((line_counter + extra_space, char_counter));
                has_galaxy = true;
            }
            col_vec.push(pixel);
        }
        if !has_galaxy {
            extra_space += 1000000 - 1;
        }
        row_vec.push(col_vec);
    }

    let galaxy_columns: HashSet<usize> = galaxy_vec.iter().map(|x| x.1).collect();

    let original_galaxy_vec = galaxy_vec.clone();

    for col_counter in 0..row_vec[0].len() {
        if galaxy_columns.contains(&(col_counter)) {
            continue;
        }

        for ((_, col), (_, old_col)) in galaxy_vec.iter_mut().zip(&original_galaxy_vec) {
            if *old_col > col_counter {
                *col = *col + 1000000 - 1;
            }
        }
    }

    return galaxy_vec;
}

// The lazy way...
fn parse_image(lines: Lines<BufReader<File>>) -> (Vec<Vec<Pixel>>, Vec<(usize, usize)>) {
    let mut row_vec = Vec::with_capacity(140);
    let mut galaxy_vec = Vec::with_capacity(1000);
    for (line_counter, line) in lines.enumerate() {
        let mut col_vec = Vec::with_capacity(140);

        let mut has_galaxy = false;
        for (char_counter, character) in line.as_ref().unwrap().chars().enumerate() {
            let pixel = Pixel::from_char(&character);

            if matches!(pixel, Pixel::Galaxy) {
                has_galaxy = true;
                galaxy_vec.push((line_counter, char_counter))
            }

            col_vec.push(pixel);
        }

        if !has_galaxy {
            row_vec.push(col_vec.clone());
        }
        row_vec.push(col_vec);
    }

    let galaxy_columns: HashSet<usize> = galaxy_vec.iter().map(|x| x.1).collect();

    let mut amount_added = 0;
    for col_counter in 0..row_vec[0].len() {
        if galaxy_columns.contains(&(col_counter)) {
            continue;
        }

        for row in row_vec.iter_mut() {
            row.insert(col_counter + amount_added, Pixel::Space);
        }
        amount_added += 1;
    }

    galaxy_vec.clear(); // Get new positions. Doing this mathematically would have been better. LOl... part 2... should have done this.
    for (row_counter, row) in row_vec.iter().enumerate() {
        for (col_counter, col) in row.iter().enumerate() {
            if matches!(col, Pixel::Galaxy) {
                galaxy_vec.push((row_counter, col_counter))
            }
        }
    }
    return (row_vec, galaxy_vec);
}
