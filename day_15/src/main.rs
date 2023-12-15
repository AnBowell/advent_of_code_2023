use std::fs;

const FILE_LOC: &'static str = "data/input.txt";
fn main() {
    problem_one();
    problem_two();
}

fn problem_one() {
    let inputs = fs::read_to_string(FILE_LOC).unwrap();
    // let inputs = "HASH";
    let mut problem_one_total = 0;
    for input in inputs.split(",") {
        problem_one_total += input
            .as_bytes()
            .iter()
            .fold(0, |acc, x| ((*x as u64 + acc) * 17) % 256);
    }
    println!("Problem one total: {}", problem_one_total)
}
fn problem_two() {
    let inputs = fs::read_to_string(FILE_LOC).unwrap();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for input in inputs.split(",") {
        let char_to_split = if input.contains("=") { "=" } else { "-" };

        let mut split_input = input.split(char_to_split);

        let code = split_input.next().unwrap();

        let focal_length = split_input.next().unwrap().parse::<usize>();

        let box_no = code
            .as_bytes()
            .iter()
            .fold(0, |acc, x| ((*x as u64 + acc) * 17) % 256);

        let this_box = boxes.get_mut(box_no as usize).unwrap();
        if char_to_split == "=" {
            if this_box.iter().filter(|x| x.0 == code).count() == 0 {
                this_box.push((code, focal_length.unwrap()));
            } else {
                let pos = this_box.iter().position(|x| x.0 == code).unwrap();
                *this_box.get_mut(pos).unwrap() = (code, focal_length.unwrap());
            }
        } else {
            if let Some(pos) = this_box.iter().position(|x| x.0 == code) {
                this_box.remove(pos);
            }
        }
    }

    let mut part_two_total = 0;
    for (box_counter, light_box) in boxes.iter().enumerate() {
        if light_box.is_empty() {
            continue;
        }

        part_two_total += light_box
            .iter()
            .enumerate()
            .map(|(pos, x)| (box_counter + 1) * (pos + 1) * x.1)
            .sum::<usize>();
    }

    println!("Part two total: {}", part_two_total);
}
