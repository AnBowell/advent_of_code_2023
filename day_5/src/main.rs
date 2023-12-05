use std::{collections::HashMap, fs::File, io::Read};

const FILE_LOC: &'static str = "data/input.txt";

type AttributeMap = HashMap<(u64, u64), (u64, u64)>;

fn main() {
    problem_one();
    problem_two();
}

fn problem_two() {
    let mut file = File::open(FILE_LOC).unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).unwrap();

    let mut input_seeds_and_maps = input_string.split("\r\n\r\n");

    let mut input_seeds = input_seeds_and_maps.next().unwrap().split_whitespace();
    input_seeds.next();
    let input_seeds = input_seeds
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let seeds_to_soil_maps = process_map_str(input_seeds_and_maps.next().unwrap());
    let soil_to_fertilizer = process_map_str(input_seeds_and_maps.next().unwrap());
    let fertilizer_to_water = process_map_str(input_seeds_and_maps.next().unwrap());
    let water_to_light = process_map_str(input_seeds_and_maps.next().unwrap());
    let light_to_temperature = process_map_str(input_seeds_and_maps.next().unwrap());
    let temperature_to_humidity = process_map_str(input_seeds_and_maps.next().unwrap());
    let humidity_to_location = process_map_str(input_seeds_and_maps.next().unwrap());

    let mut min_location = u64::MAX;

    for input_seed_index in (0..input_seeds.len()).step_by(2) {
        let max = input_seeds[input_seed_index] + input_seeds[input_seed_index + 1];

        let all_possible_numbers = input_seeds[input_seed_index]..max;

        let mut original_loc_val_diff = i64::MAX;

        let iter_step = 10_000;
        let step_by = all_possible_numbers.step_by(iter_step);

        for input_seed_val in step_by {
            // Really messy, but cba to clean it up!
            let soil_val = run_through_map(&seeds_to_soil_maps, input_seed_val);
            let fert_val = run_through_map(&soil_to_fertilizer, soil_val);
            let water_val = run_through_map(&fertilizer_to_water, fert_val);
            let light_val = run_through_map(&water_to_light, water_val);
            let temp_val = run_through_map(&light_to_temperature, light_val);
            let humid_val = run_through_map(&temperature_to_humidity, temp_val);
            let loc_val = run_through_map(&humidity_to_location, humid_val);

            let this_diff = input_seed_val as i64 - loc_val as i64;

            // If difference between last difference (input seed -> location), then process each one looking for change.

            if original_loc_val_diff != this_diff {
                if input_seed_val == input_seeds[input_seed_index] {
                    min_location = min_location.min(loc_val);
                    original_loc_val_diff = this_diff; // First one will always be different.
                    continue;
                }

                // Don't worry about u64, can't go negative here..
                for every_input_seed_val in input_seed_val - iter_step as u64..input_seed_val {
                    let soil_val = run_through_map(&seeds_to_soil_maps, every_input_seed_val);
                    let fert_val = run_through_map(&soil_to_fertilizer, soil_val);
                    let water_val = run_through_map(&fertilizer_to_water, fert_val);
                    let light_val = run_through_map(&water_to_light, water_val);
                    let temp_val = run_through_map(&light_to_temperature, light_val);
                    let humid_val = run_through_map(&temperature_to_humidity, temp_val);
                    let loc_val = run_through_map(&humidity_to_location, humid_val);
                    min_location = min_location.min(loc_val);
                }
                original_loc_val_diff = this_diff;
            }
        }
    }
    println!("Problem two next location: {}", min_location);
}

fn problem_one() {
    let mut file = File::open(FILE_LOC).unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).unwrap();

    let mut input_seeds_and_maps = input_string.split("\r\n\r\n");

    let mut input_seeds = input_seeds_and_maps.next().unwrap().split_whitespace();
    input_seeds.next(); // skip over "seeds: "

    let seeds_to_soil_maps = process_map_str(input_seeds_and_maps.next().unwrap());
    let soil_to_fertilizer = process_map_str(input_seeds_and_maps.next().unwrap());
    let fertilizer_to_water = process_map_str(input_seeds_and_maps.next().unwrap());
    let water_to_light = process_map_str(input_seeds_and_maps.next().unwrap());
    let light_to_temperature = process_map_str(input_seeds_and_maps.next().unwrap());
    let temperature_to_humidity = process_map_str(input_seeds_and_maps.next().unwrap());
    let humidity_to_location = process_map_str(input_seeds_and_maps.next().unwrap());

    let mut min_location = u64::MAX;
    for input_seed in input_seeds {
        let input_seed_val = input_seed.parse::<u64>().unwrap();

        // Could clean up into something more generic.
        let soil_val = run_through_map(&seeds_to_soil_maps, input_seed_val);
        let fert_val = run_through_map(&soil_to_fertilizer, soil_val);
        let water_val = run_through_map(&fertilizer_to_water, fert_val);
        let light_val = run_through_map(&water_to_light, water_val);
        let temp_val = run_through_map(&light_to_temperature, light_val);
        let humid_val = run_through_map(&temperature_to_humidity, temp_val);
        let loc_val = run_through_map(&humidity_to_location, humid_val);
        min_location = min_location.min(loc_val);
    }

    println!("Problem one next location: {}", min_location);
}

fn run_through_map(map: &AttributeMap, input_seed: u64) -> u64 {
    let mut mapped_val: Option<u64> = None;

    for key in map.keys() {
        if input_seed >= key.0 && input_seed < key.1 {
            mapped_val = Some(input_seed + map.get(&key).unwrap().0 - key.0)
        }
    }

    if mapped_val.is_none() {
        mapped_val = Some(input_seed);
    }

    return mapped_val.unwrap();
}
fn process_map_str(map_str: &str) -> AttributeMap {
    let mut map_iter = map_str.split("\r\n");

    map_iter.next();

    let mut hashmap = HashMap::new();

    for map_line in map_iter {
        let map_vals = map_line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        hashmap.insert(
            (map_vals[1], map_vals[1] + map_vals[2]), // Source
            (map_vals[0], map_vals[0] + map_vals[2]), // Destination
        );
    }

    return hashmap;
}
