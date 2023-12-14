use std::{
    collections::HashSet,
    fs::File,
    io::{self, stdin, stdout, BufRead, Write},
};

const FILE_LOC: &'static str = "data/test.txt";
fn main() {
    // problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let mut records_and_numbers = line.as_ref().unwrap().split_whitespace();

        let records: Vec<usize> = records_and_numbers
            .next()
            .unwrap()
            .chars()
            .map(|x| number_from_char(&x))
            .collect();

        let groups = records_and_numbers
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        println!("Records: {:?}, groups: {:?}", records, groups);
    }
}

///
/// 16384 = 4 ** 7
/// 506250 = 15 ** 5 * 10**1
/// 2500 = 4 * 5 * 5 * 5 * 5
/// 1042162054049 too low
/// 7245235336193 too high
// fn problem_two() {
//     let file = File::open(FILE_LOC).unwrap();
//     let lines = io::BufReader::new(file).lines();

//     let mut total_combinations = 0;

//     let problem_one_combinations = problem_one();

//     for (line, (problem_one_comb, problem_one_last)) in lines.zip(problem_one_combinations) {
//         let mut records_and_numbers = line.as_ref().unwrap().split_whitespace();

//         let mut records: Vec<usize> = records_and_numbers
//             .next()
//             .unwrap()
//             .chars()
//             .map(|x| number_from_char(&x))
//             .collect();
//         println!("Records: {:?}", records);
//         println!("Line length: {}", records.len());
//         // if records.last().unwrap() != &1_usize {
//         let mut records_at_start = records.clone();

//         // println!("Choose start :{:?}", !problem_one_last.contains(&0));
//         records_at_start.insert(0, 2);

//         let mut records_at_end = records.clone();
//         records_at_end.push(2);
//         // } else {
//         // records_at_end.insert(0, 2);
//         // }

//         // println!("Records START: {:?}", records);

//         if problem_one_last.contains(&0) {
//             records.push(2);
//         } else {
//             records.insert(0, 2);
//         }
//         // for x in problem_one_last {

//         //     records.insert(0, x);
//         //     break;
//         // }
//         // records.insert(1, 2);

//         // println!("Records: {:?}", records);
//         // records.insert(0, 1);

//         // }
//         // println!("Records: {:?}", records);

//         let groups = records_and_numbers
//             .next()
//             .unwrap()
//             .split(",")
//             .map(|x| x.parse::<usize>().unwrap())
//             .collect::<Vec<usize>>();

//         let groups_sum: usize = groups.iter().sum();

//         println!("Groups sum: {}", groups_sum);

//         let (amount_that_match_start, _) = calculate_combinations(&records_at_start, &groups);
//         let (amount_that_match_end, _) = calculate_combinations(&records_at_end, &groups);

//         let (amount_that_match, _) = calculate_combinations(&records, &groups);
//         // if amount_that_match == problem_one_comb {
//         //     records.remove(0);
//         //     records.push(2);
//         //     amount_that_match = calculate_combinations(&records, &groups);
//         // }
//         println!(
//             "Amount that match1, match 2 start: {},{}. Adding: {}",
//             problem_one_comb,
//             amount_that_match_start,
//             problem_one_comb * amount_that_match_start.pow(4)
//         );
//         println!(
//             "Amount that match1, match 2 end: {},{}. Adding: {}",
//             problem_one_comb,
//             amount_that_match_end,
//             problem_one_comb * amount_that_match_end.pow(4)
//         );
//         println!("Final dexciton: {}", amount_that_match);
//         // println!(
//         //     "problem_one_comb, two comb: {}, {}",
//         //     problem_one_comb, amount_that_match_groups
//         // );
//         total_combinations += problem_one_comb * amount_that_match.pow(4);
//         // let mut s = String::new();
//         // print!("Please enter some text: ");
//         // let _ = stdout().flush();
//         // stdin()
//         //     .read_line(&mut s)
//         //     .expect("Did not enter a correct string");
//     }

//     println!("Problem two total combinations: {}", total_combinations);
// }

// Takes a few moments!
fn problem_one() -> Vec<(usize, HashSet<usize>)> {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut combinations_per_line = Vec::new();
    let mut total_combinations = 0;
    for line in lines {
        let mut records_and_numbers = line.as_ref().unwrap().split_whitespace();

        let records: Vec<usize> = records_and_numbers
            .next()
            .unwrap()
            .chars()
            .map(|x| number_from_char(&x))
            .collect();

        let groups = records_and_numbers
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let amount_that_match_groups = calculate_combinations(&records, &groups);
        total_combinations += amount_that_match_groups.0;
        combinations_per_line.push(amount_that_match_groups);
        // println!("Amount that match groups: {}", amount_that_match_groups);
    }
    println!("Problem one total combinations: {}", total_combinations);
    return combinations_per_line;
}

fn calculate_combinations(records: &Vec<usize>, groups: &Vec<usize>) -> (usize, HashSet<usize>) {
    let to_replace: Vec<usize> = records
        .iter()
        .enumerate()
        .filter(|(_, x)| *x == &2_usize)
        .map(|(pos, _)| pos)
        .collect::<Vec<usize>>();

    let amount_damaged = to_replace.len();

    let mut amount_that_match_groups = 0;
    let mut hashset: HashSet<usize> = HashSet::new();
    // Find the amount of different combinations... binary.
    for i in 0..2_usize.pow(amount_damaged as u32) {
        let combination = format!("{:0fill$b}", i, fill = amount_damaged)
            .chars()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut this_records = records.clone();

        for (index, number) in to_replace.iter().zip(&combination) {
            this_records[*index] = *number;
        }

        if groups_match_records(&groups, &this_records) {
            amount_that_match_groups += 1;
            hashset.insert(*this_records.last().unwrap());
        }
        assert_eq!(combination.len(), amount_damaged)
    }

    return (amount_that_match_groups, hashset);
}

fn number_from_char(character: &char) -> usize {
    match character {
        '.' => 1,
        '#' => 0,
        '?' => 2,
        _ => panic!("Cannot get this character in file."),
    }
}

fn groups_match_records(groups: &Vec<usize>, records: &Vec<usize>) -> bool {
    let mut current_damaged_in_a_row = 0_usize;
    let mut groups_iter = groups.into_iter();

    for counter in 0..records.len() {
        let current = records[counter];
        if current == 0 {
            current_damaged_in_a_row += 1
        } else if current == 1 {
            if current_damaged_in_a_row > 0 {
                let next = groups_iter.next();

                if next != Some(&current_damaged_in_a_row) {
                    return false;
                }
            }
            current_damaged_in_a_row = 0;
        } else {
            panic!("Should have been filtered before getting here.")
        }

        // Flush if on last one.
        if counter == records.len() - 1 {
            if current_damaged_in_a_row > 0 {
                let next = groups_iter.next();
                if next != Some(&current_damaged_in_a_row) {
                    return false;
                }
            }
        }
    }

    if groups_iter.next().is_some() {
        return false;
    }
    // println!("Records: {:?}", records);
    return true;
}

#[test]
fn test_group_match_records() {
    let records = vec![0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1];
    let groups = vec![4, 1, 1];

    let res = groups_match_records(&groups, &records);

    println!("Res: {}", res);
}
