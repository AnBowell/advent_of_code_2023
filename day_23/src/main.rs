use core::panic;
use std::{
    collections::{self, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    vec, ops::Range, env, thread::current,
};
const FILE_LOC: &'static str = "data/test.txt";

fn main() {
    problem_one();

}




fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let mut reader = io::BufReader::new(file);


    let (env, start_pos, finish_pos) = parse_problem(&mut reader);


    let mut positions = vec![start_pos];
    let mut directions = vec![Direction::Down];
    let mut current_steps = vec![0];

    // let mut position = start_pos.clone();
    // let mut direction = Direction::Down;
    // let mut number_of_steps = 0;


    while positions.iter().filter(|x| **x != finish_pos).count() > 0{

        let mut all_new_positions = Vec::new();
        let mut all_new_direcitons = Vec::new();
        let mut all_new_current_steps = Vec::new();

        println!("Positions: {:?}", positions);
        // let mut all new_steps = Vec::new
        for ((position, direction), step) in positions.iter().zip(&directions).zip(&current_steps){

            if *position == finish_pos{
                continue
            }
            println!("Position, direction: {:?}, {:?}", position, direction);
            let (mut new_positions, mut new_directions) = find_possible_directions(&env,*position,
                direction);

            if !new_positions.is_empty(){
                all_new_current_steps.append(&mut vec![step+1; new_directions.len()]);
            } else{
                all_new_current_steps.append(&mut vec![*step; new_directions.len()])
            }

            all_new_positions.append(&mut new_positions);
            all_new_direcitons.append(&mut new_directions);
     

        }

        positions = all_new_positions;
        directions = all_new_direcitons;
        current_steps = all_new_current_steps;


        println!("Positions, directions: {:?}, {:?}", positions, directions);
        // *positions.get_mut(0).unwrap() = *new_positions.last().unwrap();
        // *directions.get_mut(0).unwrap() = new_directions.last().unwrap().to_owned();
        // *current_steps.get_mut(0).unwrap() +=1;

        
      
    }

    println!("Number of steps taken: {:?}", current_steps);

    // println!("Start pos: {:?}", env);

}


fn parse_problem(reader: &mut BufReader<File>) -> (Vec<Vec<Enviroment>>, (usize, usize), (usize, usize)) {

    let mut start: Option<(usize, usize)> = None;
    let mut end = (0,0);
    let mut env_vec = Vec::with_capacity(200);
   
    for (line_counter, line) in reader.lines().enumerate(){

        let row_vec = line.as_ref().unwrap().chars().map(|x| Enviroment::from_char(&x)).collect::<Vec<Enviroment>>();


        if start.is_none(){
            start = Some((0, row_vec.iter().position(|x| matches!(x, Enviroment::Path)).unwrap()));
        }

        end = (line_counter, row_vec.iter().position(|x| matches!(x, Enviroment::Path)).unwrap()); // 


        env_vec.push(row_vec);

    }

    return (env_vec, start.unwrap(),end )
}


fn find_possible_directions(environment: &Vec<Vec<Enviroment>>, current_loc: (usize, usize), last_direction: &Direction) -> (Vec<(usize, usize)>, Vec<Direction>){


    let mut positions =  Vec::new();
    let mut directions = Vec::new();

    for direction in Direction::as_vec(){

        if direction == last_direction.opposite(){
            continue;
        }

        let next_step = direction.next_step(current_loc);

        match &environment[next_step.0][next_step.1]{
            Enviroment::Path => directions.push(direction.to_owned()),
            Enviroment::Forest => continue,
            Enviroment::Slope(slope_dir) => if *slope_dir == direction.opposite(){
                continue
            } else{
                directions.push(slope_dir.to_owned())
            }
            
             ,
        }

        positions.push(next_step);

    }

    return (positions, directions)


}


#[derive(Debug, Clone)]
enum Enviroment{
    Path,
    Forest,
    Slope(Direction)
}
impl Enviroment{
    fn from_char(character: &char) -> Self{
        match character{
            '.' => Self::Path,
            '#' => Self::Forest,
            '>' => Self::Slope(Direction::Right),
            '<' => Self::Slope(Direction::Left),
            '^' => Self::Slope(Direction::Up),
            'v' => Self::Slope(Direction::Down),
            _ => panic!("Invalid Character!")
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    fn as_vec() -> Vec<Self>{
     vec![Self::Up, Self::Down, Self::Left, Self::Right]   
    }
    fn opposite(&self) -> Self{
        match self{
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
        }
    }
    fn next_step(&self, current_pos: (usize, usize)) -> (usize, usize){
        match self{
            Direction::Up => (current_pos.0 -1, current_pos.1),
            Direction::Down => (current_pos.0 + 1, current_pos.1),
            Direction::Left => (current_pos.0, current_pos.1 -1),
            Direction::Right => (current_pos.0, current_pos.1 + 1),
        }
    }
}