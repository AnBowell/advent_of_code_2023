use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

const FILE_LOC: &'static str = "data/input.txt";
fn main() {
    problem_one();
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (row_valleys, column_valleys) = parse_valley(lines);

    let mut total = 0;
    for (row_valley, column_valley) in row_valleys.iter().zip(column_valleys) {
        let mirror_loc = scan_valley(&row_valley, &column_valley);

        match mirror_loc {
            RowColumn::Row(x, _) => total += 100 * (x + 1),
            RowColumn::Column(x, _) => total += x + 1,
        }
    }
    println!("Problem one total: {}", total);
}

fn scan_valley(row_vec: &Vec<Vec<char>>, column_vec: &Vec<Vec<char>>) -> RowColumn {
    let mut possible_symmetries = Vec::new();

    for i in 0..row_vec.len() - 1 {
        let row_one = &row_vec[i];
        let row_two = &row_vec[i + 1];

        if row_one == row_two {
            possible_symmetries.push(RowColumn::Row(i, i + 1))
        }
    }
    for i in 0..column_vec.len() - 1 {
        let column_one = &column_vec[i];
        let column_two = &column_vec[i + 1];
        if column_one == column_two {
            possible_symmetries.push(RowColumn::Column(i, i + 1))
        }
    }

    let mut symmetry_counter = vec![0; possible_symmetries.len()];
    for (counter, symmetry) in possible_symmetries.iter().enumerate() {
        let vec_to_use = match symmetry {
            RowColumn::Row(_, _) => &row_vec,
            RowColumn::Column(_, _) => &column_vec,
        };
        let indicies = symmetry.get_indicies();

        let left_and_right = indicies.0.min(vec_to_use.len() - 1 - indicies.1);

        if left_and_right == 0 {
            symmetry_counter[counter] = 1;
            continue;
        }
        for add_counter in 1..=left_and_right {
            if vec_to_use[indicies.0 - add_counter] == vec_to_use[indicies.1 + add_counter] {
                symmetry_counter[counter] += 1;
            } else {
                symmetry_counter[counter] = 0;
                break;
            }
        }
    }

    let max = symmetry_counter
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();

    return possible_symmetries[max].clone();
}

#[derive(Debug, Clone)]
enum RowColumn {
    Row(usize, usize),
    Column(usize, usize),
}
impl RowColumn {
    fn get_indicies(&self) -> (usize, usize) {
        return match self {
            RowColumn::Row(x, y) => (*x, *y),
            RowColumn::Column(x, y) => (*x, *y),
        };
    }
}

// lots of cloning here, could be better...
fn parse_valley(lines: Lines<BufReader<File>>) -> (Vec<Vec<Vec<char>>>, Vec<Vec<Vec<char>>>) {
    let mut pattern_vec = Vec::new();
    let mut columnar_pattern_vec = Vec::new();
    let mut row_vec = Vec::new();

    for line in lines {
        let chars = line.as_ref().unwrap().chars().collect::<Vec<char>>();

        if chars.len() == 0 {
            let column_vec = transpose(row_vec.clone());

            pattern_vec.push(row_vec.clone());
            columnar_pattern_vec.push(column_vec);
            row_vec.clear();

            continue;
        }

        row_vec.push(chars);
    }

    let column_vec = transpose(row_vec.clone());
    columnar_pattern_vec.push(column_vec);
    pattern_vec.push(row_vec);

    return (pattern_vec, columnar_pattern_vec);
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
