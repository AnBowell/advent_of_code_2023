use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

const FILE_LOC: &'static str = "data/test.txt";

fn main() {
    problem_one();
    problem_two();
}

struct Tree {
    nodes: Vec<Tree>,
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (rules, _) = parse_problem(lines);

    let mut problem_one_total = 0;
    let mut start = 1e15 as u64;

    let start_rule = rules.get("in").unwrap();

    let mut tracker = HashMap::new();

    start_rule.process_parts(&rules, &mut start, &mut tracker);

    println!("Problem two total: {}", problem_one_total);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (rules, parts) = parse_problem(lines);

    let mut problem_one_total = 0;
    let start_rule = rules.get("in").unwrap();

    for part in parts {
        let mut accepted_or_not = String::new();

        start_rule.check_part(&part, &rules, &mut accepted_or_not);

        if accepted_or_not == "A" {
            problem_one_total += part.sum();
        }
    }

    println!("Problem one total: {}", problem_one_total);
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Option<(i64, char, i64, String)>>,
    default: String,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            default: String::new(),
        }
    }
}

impl Rules {
    fn process_parts(
        &self,
        rule_map: &HashMap<String, Rules>,
        current_count: &mut u64,
        tracker: &mut HashMap<String, (Part, Part)>,
    ) {
        let mut rules = self.rules.clone();
        rules.sort_by_key(|x| x.clone().map(|x| x.0));
        rules.reverse();

        for rule in rules {
            if rule.is_none() {
                if &self.default == "A" || &self.default == "R" {
                    // *to_go_to = self.default.clone();
                    println!("End of the line!: {:?}", &self.default);
                    return;
                }
                println!("Rule: {}", self.default);

                rule_map.get(&self.default).unwrap().process_parts(
                    rule_map,
                    current_count,
                    tracker,
                );

                println!("Stepping back up");
                return;
            }

            let condition = true;
            if condition {
                if &rule.as_ref().unwrap().3 == "A" || &rule.as_ref().unwrap().3 == "R" {
                    // *to_go_to = rule.as_ref().unwrap().3.clone();
                    println!("End of the line!: {:?}", &rule.as_ref().unwrap().3);
                    continue;
                }
                println!("Rule: {}", &rule.as_ref().unwrap().3);

                let entry = tracker
                    .entry(self.default.clone())
                    .or_insert((Part::min(), Part::max()));

                match &rule.as_ref().unwrap().1 {
                    'x' => if &rule.as_ref().unwrap().2 < 0{&rule.as_ref().unwrap().1},
                    'm' => ,
                    'a' => ,
                    's' => ,
                    _ => panic!("Invalid char")
                }

                rule_map
                    .get(&rule.as_ref().unwrap().3)
                    .unwrap()
                    .process_parts(rule_map, current_count, tracker);
            }
        }

        return;
    }

    fn check_part(&self, part: &Part, rule_map: &HashMap<String, Rules>, to_go_to: &mut String) {
        let mut rules = self.rules.clone();
        rules.sort_by_key(|x| x.clone().map(|x| x.0));
        rules.reverse();

        for rule in rules {
            if rule.is_none() {
                if &self.default == "A" || &self.default == "R" {
                    *to_go_to = self.default.clone();
                    return;
                }

                rule_map
                    .get(&self.default)
                    .unwrap()
                    .check_part(part, rule_map, to_go_to);
                return;
            }

            let part_value = match rule.as_ref().unwrap().1 {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!("Ahhh. Not in Xmas."),
            };

            let condition = if rule.as_ref().unwrap().2 < 0 {
                part_value > rule.as_ref().unwrap().2.abs()
            } else {
                part_value < rule.as_ref().unwrap().2
            };

            if condition {
                if &rule.as_ref().unwrap().3 == "A" || &rule.as_ref().unwrap().3 == "R" {
                    *to_go_to = rule.as_ref().unwrap().3.clone();
                    return;
                }

                rule_map
                    .get(&rule.as_ref().unwrap().3)
                    .unwrap()
                    .check_part(part, rule_map, to_go_to);
                return;
            }
        }

        return;
    }

    fn from_string(rule_string: &str) -> (String, Self) {
        let mut split_string = rule_string.split("{");

        let name = split_string.next().unwrap().to_string();

        let rules = split_string.next().unwrap().replace("}", "");
        let rules = rules.split(",");

        let len = rules.clone().count();

        let mut rule_struct = Rules::default();

        for (rule_no, rule) in rules.enumerate() {
            if !rule.contains(":") {
                rule_struct.default = rule.to_string();
                continue;
            }
            let mut rule_split = rule.split(":");

            let condition = rule_split.next().unwrap();
            let result = rule_split.next().unwrap().to_string();

            let mut characters = condition.chars();

            let property = characters.next().unwrap();

            let operator = match characters.next().unwrap() {
                '<' => 1,
                '>' => -1,
                _ => panic!("Invalid operator"),
            };

            let mut value = String::new();
            while let Some(i) = characters.next() {
                value.push(i)
            }

            let value = value.parse::<i64>().unwrap();

            rule_struct.rules.push(Some((
                (len - rule_no) as i64,
                property,
                value * operator,
                result,
            )));
        }
        rule_struct.rules.push(None);

        return (name, rule_struct);
    }
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn max() -> Self {
        Self {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        }
    }
    fn min() -> Self {
        Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        }
    }
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
    fn from_string(part_string: &str) -> Self {
        let mut x_m_a_s_vals = part_string.split(",").map(|x| {
            x.split("=")
                .filter(|x| x.replace("}", "").parse::<i64>().is_ok())
                .next()
                .unwrap()
                .replace("}", "")
                .parse::<i64>()
                .unwrap()
        });

        Self {
            x: x_m_a_s_vals.next().unwrap(),
            m: x_m_a_s_vals.next().unwrap(),
            a: x_m_a_s_vals.next().unwrap(),
            s: x_m_a_s_vals.next().unwrap(),
        }
    }
}

fn parse_problem(lines: Lines<BufReader<File>>) -> (HashMap<String, Rules>, Vec<Part>) {
    let mut parts_processing = false;

    let mut parts = Vec::with_capacity(200);
    let mut rules = HashMap::new();

    for line in lines {
        let content = line.as_ref().unwrap();

        // Blank line in file between switch to parts.
        if content.is_empty() {
            parts_processing = true;
            continue;
        }

        if parts_processing {
            parts.push(Part::from_string(&content));
        } else {
            let (name, rule) = Rules::from_string(&content);
            rules.insert(name, rule);
        }
    }

    return (rules, parts);
}
