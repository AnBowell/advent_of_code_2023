/// Quite a messy one today... Solution was nice and extendable for second part but code could have
/// been a lot cleaner.


use std::{io::{self, BufRead}, fs::File, collections::HashSet};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two();
}
#[derive(Debug, Clone)]
enum EngineSchematic{
    Number((i64, usize)), // Number and length.
    NumberFiller(usize), // How many elements left number is.
    Symbol(char),
    Dot,
}


fn problem_two(){
    let file = File::open(FILE_LOC).unwrap();
    let engine_lines = process_text_to_vec(&file);
    let  mut total_part_sum = 0;
    for row_counter in 0..engine_lines.len(){

      for column_counter in  0..engine_lines.first().unwrap().len(){
            
            let symbol = match engine_lines[row_counter][column_counter] {
                EngineSchematic::Number(_) => continue,
                EngineSchematic::Symbol(symbol) => symbol,
                EngineSchematic::Dot =>  continue,
                EngineSchematic::NumberFiller(_) => continue,
            };

            if symbol != '*'{
                continue;
            }

            let symbol_chec =check_for_surrounding_number(&engine_lines, row_counter as i64, column_counter as i64);
          
            match symbol_chec{
                Some(adjacent_nums) => {
                    // Deduplicate items.
                    let mut unique_items: HashSet<_> = adjacent_nums.iter().cloned().collect();
                    if unique_items.len() != 2{
                        continue;
                    }
                    let multiply_iter =unique_items.drain().fold(1, |acc, x| acc * x);

                    total_part_sum += multiply_iter;
                },
                None => (),
            }
            
        }
    }


    println!("Total part sum: {}", total_part_sum);

}
fn check_for_surrounding_number(vec: &Vec<Vec<EngineSchematic>>, row: i64, column: i64) -> Option<Vec<i64>>{
    let default_vec = Vec::new();
    let mut return_vec = Vec::new();
    for x in -1..=1{
        for y in -1..=1{

            if !(row + x >= 0 && column + y >= 0){
                continue;
            }

            let Some(en) = vec.get((row+x) as usize).unwrap_or(&default_vec).get((column+y) as usize) else{
                continue
            };

            match en{
                EngineSchematic::Number((num, _)) => return_vec.push(*num),
                EngineSchematic::Symbol(_) =>continue,
                EngineSchematic::Dot => continue,
                EngineSchematic::NumberFiller(left) => return_vec.push(
                    match vec.get((row+x) as usize).unwrap_or(&default_vec).get((column+y) as usize - left).unwrap().clone(){
                        EngineSchematic::Number((num, _)) =>num,
                        _ => panic!("Not possible!")
                    }
                )
            }
        }
        }    
    return Some(return_vec)

             
}


fn problem_one(){
    let file = File::open(FILE_LOC).unwrap();
    let engine_lines = process_text_to_vec(&file);
    let  mut total_part_sum = 0;
    for row_counter in 0..engine_lines.len(){

      for column_counter in  0..engine_lines.first().unwrap().len(){
            
            let (part_number, number_length) = match engine_lines[row_counter][column_counter] {
                EngineSchematic::Number((part_number, number_length)) => (part_number, number_length),
                EngineSchematic::Symbol(_) => continue,
                EngineSchematic::Dot =>  continue,
                EngineSchematic::NumberFiller(_) => continue,
            };

            let symbol_chec =check_surrounding_elements(&engine_lines, row_counter as i64, column_counter as i64, number_length);

            match symbol_chec{
                Some(_) => {
                    total_part_sum+= part_number
                },
                None => (),
            }
            
        }
    }


    println!("Total part sum: {}", total_part_sum);

}

fn check_surrounding_elements(vec: &Vec<Vec<EngineSchematic>>, row: i64, column: i64, length: usize) -> Option<EngineSchematic>{
    let default_vec = Vec::new();

    for x in -1..=1{
        for y in -1..=length as i64{

            if !(row + x >= 0 && column + y >= 0){
                continue;
            }

            let Some(en) = vec.get((row+x) as usize).unwrap_or(&default_vec).get((column+y) as usize) else{
                continue
            };

            return match en{
                EngineSchematic::Number(_) => continue,
                EngineSchematic::Symbol(_) =>Some(en.clone()),
                EngineSchematic::Dot => continue,
                EngineSchematic::NumberFiller(_) => continue,
            }
        }
        }    
    return None

           
            
}



fn process_text_to_vec(file: &File) -> Vec<Vec<EngineSchematic>>{
    let lines = io::BufReader::new(file).lines();

    let mut engine_lines = Vec::new();
    for line in lines{
        let mut engine_line = Vec::new();
        let mut number_string = String::new();
        for character in line.unwrap().chars(){

            if character.is_numeric(){
                number_string.push(character)
            } else{
                if !number_string.is_empty(){
                    let number_length = number_string.len();
                    engine_line.push(EngineSchematic::Number((number_string.parse().unwrap(), number_length)));
                    for i in 1..number_length {
                        engine_line.push(EngineSchematic::NumberFiller(i));
                    }
                    number_string.clear();
                }
                if character == '.'{
                    engine_line.push(EngineSchematic::Dot);
                } else{
                    engine_line.push(EngineSchematic::Symbol(character));
                };
            }
        }
        if !number_string.is_empty(){
            // Make function... 
            let number_length = number_string.len();
            engine_line.push(EngineSchematic::Number((number_string.parse().unwrap(), number_string.len()))); // Flush numbers at end of lines
            for i in 1..number_length {
                engine_line.push(EngineSchematic::NumberFiller(i));
            }
        }
        engine_lines.push(engine_line);
    }
    return engine_lines
}