use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Iter,
};

const FILE_LOC: &'static str = "data/test.txt";

fn main() {
    problem_one();
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (rules, parts) = parse_problem(lines);

    for part in parts {
        let current_rule = rules.get("in").unwrap();

        let mut accepted_or_not = String::new();
        current_rule.check_part(&part, &rules, &mut accepted_or_not);

        println!("Accepted or not: {}", accepted_or_not);
    }

    // println!("{:?}", rules);
}

fn parse_problem(lines: Lines<BufReader<File>>) -> (HashMap<String, Rule>, Vec<Part>) {
    // let rule_hashmap: HashMap<String, > = HashMap::new();

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
            let (name, rule) = Rule::from_string(&content);
            rules.insert(name, rule);
        }
    }

    return (rules, parts);
}

#[derive(Debug)]
struct Rule {
    x: Option<(i64, char, i64, String)>,
    m: Option<(i64, char, i64, String)>,
    a: Option<(i64, char, i64, String)>,
    s: Option<(i64, char, i64, String)>,
    default: String,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            x: None,
            m: None,
            a: None,
            s: None,
            default: String::new(),
        }
    }
}

impl Rule {
    fn check_part(
        &self,
        part: &Part,
        rule_map: &HashMap<String, Rule>,
        to_go_to: &mut String,
    ) -> String {
        if to_go_to == "A" || to_go_to == "R" {
            return to_go_to.clone();
        }

        let mut rules = self.as_vec();
        rules.sort_by_key(|x| x.clone().map(|x| x.0));
        rules.reverse();

        for rule in rules {
            if rule.is_none() {
                *to_go_to = rule_map
                    .get(&self.default)
                    .unwrap()
                    .check_part(part, rule_map, to_go_to);
                continue;
            }

            let part_value = match rule.as_ref().unwrap().1 {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!("Ahhh. Not in Xmas."),
            };
            println!("{:?}", &rule.as_ref().unwrap().3);
            if part_value - rule.as_ref().unwrap().2 < 0 {
                *to_go_to = rule_map
                    .get(&rule.as_ref().unwrap().3)
                    .unwrap()
                    .check_part(part, rule_map, to_go_to);
            }
        }

        return to_go_to.to_string();
    }

    fn as_vec(&self) -> Vec<Option<(i64, char, i64, String)>> {
        vec![
            self.x.clone(),
            self.m.clone(),
            self.a.clone(),
            self.s.clone(),
        ]
    }
    fn from_string(rule_string: &str) -> (String, Self) {
        let mut split_string = rule_string.split("{");

        let name = split_string.next().unwrap().to_string();

        let rules = split_string.next().unwrap().replace("}", "");
        let rules = rules.split(",");

        let mut rule_struct = Rule::default();

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

            match property {
                'x' => rule_struct.x = Some((rule_no as i64, property, value * operator, result)),
                'm' => rule_struct.m = Some((rule_no as i64, property, value * operator, result)),
                'a' => rule_struct.a = Some((rule_no as i64, property, value * operator, result)),
                's' => rule_struct.s = Some((rule_no as i64, property, value * operator, result)),
                _ => panic!("Ahhh. Not in Xmas."),
            }
        }

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
    fn as_vec(&self) -> Vec<i64> {
        vec![self.x, self.m, self.a, self.s]
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
