use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    vec,
};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (destinations, mut modules) = parse_problem(lines);

    let previous_destinations = vec!["broadcaster".to_string(); destinations.len()];

    let current_pulses = vec![Pulse::Low; destinations.len()];

    let mut button_presses = 0_u64;

    let mut total_steps = Vec::new();

    let mut total_steps_tracker = HashSet::new();

    'outer: loop {
        button_presses += 1;
        let mut destinations_this_iter = destinations.clone();
        let mut previous_destinations_this_iter = previous_destinations.clone();
        let mut current_pulses_this_iter = current_pulses.clone();

        while !destinations_this_iter.is_empty() && !current_pulses_this_iter.is_empty() {
            let mut to_replace_destinations = Vec::new();
            let mut to_replace_pulses = Vec::new();
            let mut to_replace_previous_destinations = Vec::new();

            for ((dest, pulse), previous_dest) in destinations_this_iter
                .iter()
                .zip(&current_pulses_this_iter)
                .zip(&previous_destinations_this_iter)
            {
                // println!("new_dests: {:?}, pulse: {:?}", new_dests, pulse);
                if dest == &"rx".to_string() {
                    if matches!(pulse, Pulse::Low) {
                        println!("Found a  pulse going to rx! {}", button_presses);
                    }
                    let module = &modules.get(previous_dest).unwrap().0;

                    match module {
                        Module::FlipFlop(_) => (),
                        Module::Conjunction(x) => {
                            let no = x.last_sent.values().position(|x| matches!(x, Pulse::High));

                            match no {
                                Some(x) => {
                                    if !total_steps.contains(&button_presses) {
                                        let not_seen_before = total_steps_tracker.insert(x);

                                        if !not_seen_before {
                                            break 'outer;
                                        }
                                        total_steps.push(button_presses);
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                }
                let (ref mut module, new_dests) = match modules.get_mut(dest) {
                    Some(x) => x,
                    None => {
                        continue;
                    }
                };

                let output = module.send(pulse, previous_dest);

                // If flop flop returns none we're done with this path
                if output.is_none() {
                    continue;
                }
                to_replace_previous_destinations.append(&mut vec![dest.clone(); new_dests.len()]);
                to_replace_pulses.append(&mut vec![output.unwrap(); new_dests.len()]);
                to_replace_destinations.append(&mut new_dests.clone());
            }

            previous_destinations_this_iter = to_replace_previous_destinations;
            destinations_this_iter = to_replace_destinations;
            current_pulses_this_iter = to_replace_pulses;
        }
    }

    let mut current_lcm = (total_steps[0] * total_steps[1]) / gcd(total_steps[0], total_steps[1]);

    for counter in 1..total_steps.len() {
        current_lcm = (total_steps[counter] * current_lcm) / gcd(total_steps[counter], current_lcm);
    }

    println!("Problem two button presses: {}", current_lcm);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut r;
    while a % b > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    return b;
}

// Should have set up a proper tree structure me thinks..
fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();

    let (destinations, mut modules) = parse_problem(lines);

    let mut high_pulse_counter = 0;
    let mut low_pulse_counter = 0;

    let previous_destinations = vec!["broadcaster".to_string(); destinations.len()];

    let current_pulses = vec![Pulse::Low; destinations.len()];

    for _ in 0..1000 {
        low_pulse_counter += 1; // Start at one for button -> broadcaster.
        let mut destinations_this_iter = destinations.clone();
        let mut previous_destinations_this_iter = previous_destinations.clone();
        let mut current_pulses_this_iter = current_pulses.clone();

        while !destinations_this_iter.is_empty() && !current_pulses_this_iter.is_empty() {
            let mut to_replace_destinations = Vec::new();
            let mut to_replace_pulses = Vec::new();
            let mut to_replace_previous_destinations = Vec::new();

            for ((dest, pulse), previous_dest) in destinations_this_iter
                .iter()
                .zip(&current_pulses_this_iter)
                .zip(&previous_destinations_this_iter)
            {
                match pulse {
                    Pulse::High => high_pulse_counter += 1,
                    Pulse::Low => low_pulse_counter += 1,
                }

                let (ref mut module, new_dests) = match modules.get_mut(dest) {
                    Some(x) => x,
                    None => {
                        continue;
                    }
                };

                let output = module.send(pulse, previous_dest);

                // If flop flop returns none we're done with this path
                if output.is_none() {
                    continue;
                }
                to_replace_previous_destinations.append(&mut vec![dest.clone(); new_dests.len()]);
                to_replace_pulses.append(&mut vec![output.unwrap(); new_dests.len()]);
                to_replace_destinations.append(&mut new_dests.clone());
            }

            previous_destinations_this_iter = to_replace_previous_destinations;
            destinations_this_iter = to_replace_destinations;
            current_pulses_this_iter = to_replace_pulses;
        }
    }
    println!(
        "Low pulses: {}\nHigh pulses: {}\nProblem one total:{}",
        low_pulse_counter,
        high_pulse_counter,
        high_pulse_counter * low_pulse_counter
    );
}

#[derive(Clone, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}
impl Module {
    fn send(&mut self, pulse: &Pulse, name: &String) -> Option<Pulse> {
        match self {
            Module::FlipFlop(x) => x.send(pulse, name),
            Module::Conjunction(x) => x.send(pulse, name),
        }
    }

    fn print_elements(&self) {
        match self {
            Module::FlipFlop(x) => println!("Flop flop"),
            Module::Conjunction(x) => println!("{:?}", x.last_sent),
        }
    }
    fn count_elements(&self) -> i64 {
        match self {
            Module::FlipFlop(_) => -1,
            Module::Conjunction(x) => x
                .last_sent
                .values()
                .filter(|x| matches!(x, Pulse::High))
                .count() as i64,
        }
    }
}

#[derive(Clone, Debug)]
struct FlipFlop {
    on: bool,
}

impl Default for FlipFlop {
    fn default() -> Self {
        Self { on: false }
    }
}

impl FlipFlop {
    fn send(&mut self, pulse: &Pulse, name: &String) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                return if self.on {
                    self.on = false;
                    Some(Pulse::Low)
                } else {
                    self.on = true;
                    Some(Pulse::High)
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
struct Conjunction {
    last_sent: HashMap<String, Pulse>,
}

impl Default for Conjunction {
    fn default() -> Self {
        Self {
            last_sent: HashMap::new(),
        }
    }
}

impl Conjunction {
    fn send(&mut self, pulse: &Pulse, name: &String) -> Option<Pulse> {
        *self.last_sent.get_mut(name).unwrap() = pulse.to_owned();

        return if self
            .last_sent
            .values()
            .filter(|x| matches!(x, Pulse::Low))
            .count()
            > 0
        {
            Some(Pulse::High)
        } else {
            Some(Pulse::Low)
        };
    }
}

fn parse_problem(
    lines: Lines<BufReader<File>>,
) -> (Vec<String>, HashMap<String, (Module, Vec<String>)>) {
    let mut broadcaster_dests = Vec::new();
    let mut module_hashmap: HashMap<_, _> = HashMap::new();

    let mut conjunctions_to_populate = HashMap::new();
    for line in lines {
        let mut module_destinations = line.as_ref().unwrap().split(" -> ");

        let module = module_destinations.next().unwrap();
        let destinations = module_destinations
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if module == "broadcaster" {
            broadcaster_dests = destinations;
            continue;
        }

        let module_name = &module[1..];

        let module = match module.chars().next().unwrap() {
            '%' => (Module::FlipFlop(FlipFlop::default()), destinations),
            '&' => {
                conjunctions_to_populate.insert(module_name.to_string(), Vec::<String>::new());
                (Module::Conjunction(Conjunction::default()), destinations)
            }
            _ => panic!("Not a valid module!"),
        };

        module_hashmap.insert(module_name.to_string(), module);
    }

    for (key, value) in module_hashmap.iter() {
        for dest in &value.1 {
            match conjunctions_to_populate.get_mut(dest) {
                Some(x) => x.push(key.to_owned()),
                None => (),
            }
        }
    }

    for (key, value) in conjunctions_to_populate.iter() {
        match module_hashmap.get_mut(key).unwrap().0 {
            Module::FlipFlop(_) => panic!("Can't be here"),
            Module::Conjunction(ref mut x) => {
                for val in value {
                    x.last_sent.insert(val.clone(), Pulse::Low);
                }
            }
        }
    }

    return (broadcaster_dests, module_hashmap);
}
