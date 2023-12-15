use std::{
    collections::HashMap,
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

    let mut problem_two_total = 0;
    for line in lines {
        let mut line_split = line.as_ref().unwrap().split_ascii_whitespace();

        let patterns = line_split.next().unwrap().chars().collect::<Vec<char>>();
        let mut extended_patterns = Vec::with_capacity(patterns.len() * 5);

        for _ in 0..5 {
            extended_patterns.append(&mut patterns.clone());
            extended_patterns.push('?');
        }
        extended_patterns.pop();

        let groups = line_split
            .next()
            .unwrap()
            .split(",")
            .map(|x: &str| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut extended_groups = Vec::with_capacity(groups.len() * 5);
        for _ in 0..5 {
            extended_groups.append(&mut groups.clone());
        }
        let mut cache = HashMap::new();
        problem_two_total +=
            calculate_combinations(&extended_patterns, &extended_groups, &mut cache);
    }
    println!("Problem two total: {}", problem_two_total);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut problem_one_total = 0;
    for line in lines {
        let mut line_split = line.as_ref().unwrap().split_ascii_whitespace();

        let patterns = line_split.next().unwrap().chars().collect::<Vec<char>>();

        let groups = line_split
            .next()
            .unwrap()
            .split(",")
            .map(|x: &str| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut cache = HashMap::new();
        problem_one_total += calculate_combinations(&patterns, &groups, &mut cache);
    }
    println!("Problem one total: {}", problem_one_total);
}

fn calculate_combinations(
    pattern: &[char],
    groups: &[usize],
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if let Some(cached_res) = cache.get(&(pattern.to_vec(), groups.to_vec())) {
        return *cached_res;
    }

    if groups.is_empty() {
        return !pattern.contains(&'#') as usize;
    }

    let remaining = groups.iter().sum::<usize>() + groups.len() - 1;

    if pattern.len() < remaining {
        return 0;
    }

    let calc_res = match pattern[0] {
        '.' => calculate_combinations(&pattern[1..], groups, cache),
        '#' => calculate_hash(pattern, groups, cache),
        '?' => {
            calculate_combinations(&pattern[1..], groups, cache)
                + calculate_hash(pattern, groups, cache)
        }
        _ => panic!("Ahhh not allowed."),
    };
    cache.insert((pattern.to_vec(), groups.to_vec()), calc_res);
    return calc_res;
}

fn calculate_hash(
    pattern: &[char],
    groups: &[usize],
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if pattern.len() < groups[0] || pattern[0..groups[0]].contains(&'.') {
        return 0;
    }

    if pattern.len() == groups[0] {
        return (groups.len() == 1) as usize;
    }

    if pattern[groups[0]] == '#' {
        return 0;
    }

    return calculate_combinations(&pattern[groups[0] + 1..], &groups[1..], cache);
}
