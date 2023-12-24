    
use std::{

    fs::File,
    io::{self, BufRead, BufReader},
};
const FILE_LOC: &'static str = "data/input.txt";

const BOUNDARY_LOWER: f64 = 200000000000000.0;
const BOUNDARY_UPPER: f64 = 400000000000000.0;


fn main() {
    problem_one();

}




fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let mut reader = io::BufReader::new(file);


    let (positions, velocities) = parse_problem(&mut reader);

    let mut number_of_interesections = 0;


    for (a_counter, (pos_a, vel_a)) in positions.iter().zip(&velocities).enumerate(){
        
        for b_counter in a_counter..positions.len(){ 
            let pos_b = &positions[b_counter];
            let vel_b = &velocities[b_counter];

            if pos_a == pos_b{
                continue;
            }
 
            let pos_a_x0 = pos_a[0];
            let pos_a_x1 = pos_a[0] + vel_a[0] * 1e16 as i64;

            let pos_b_x0 = pos_b[0];
            let pos_b_x1 = pos_b[0] + vel_b[0] * 1e16 as i64;

            let pos_a_y0 = pos_a[1];
            let pos_a_y1 = pos_a[1] + vel_a[1] * 1e16 as i64;

            let pos_b_y0 = pos_b[1];
            let pos_b_y1 = pos_b[1] + vel_b[1] * 1e16 as i64;

            println!("pos b y1: {}", pos_b_y1);

            let intersect = line_intersection(
                pos_a_x0 as f64,
                 pos_a_y0 as f64, 
                 pos_a_x1 as f64,
                  pos_a_y1 as f64, 
                  pos_b_x0 as f64, 
                  pos_b_y0 as f64,
                   pos_b_x1 as f64, 
                   pos_b_y1 as f64);
        
            if intersect.is_none(){
                continue
            }

            let intersect = intersect.unwrap();

            if intersect.0 < BOUNDARY_LOWER || intersect.0 > BOUNDARY_UPPER || intersect.1 < BOUNDARY_LOWER || intersect.1 > BOUNDARY_UPPER{
                continue;   
            } 
  

            number_of_interesections+=1;

            
            println!("Interesect :{:?}", intersect);
        }
    }

    println!("Problem one total: {}", number_of_interesections);

}




fn line_intersection(line_x0: f64, line_y0: f64, line_x1: f64, line_y1: f64,
line2_x0: f64, line2_y0: f64, line2_x1: f64, line2_y1: f64) -> Option<(f64, f64)>{


    let s1_x = line_x1 - line_x0;
    let s1_y = line_y1 - line_y0;

    let s2_x = line2_x1 - line2_x0;
    let s2_y = line2_y1 - line2_y0;


    let s = (-s1_y * (line_x0 - line2_x0) + s1_x * (line_y0 - line2_y0)) / (-s2_x * s1_y + s1_x * s2_y);
    let t = ( s2_x * (line_y0 - line2_y0) - s2_y * (line_x0 - line2_x0)) / (-s2_x * s1_y + s1_x * s2_y);


    if s>= 0.0 && s <= 1.0 && t >= 0.0 && t <=1.0{
        return Some((line_x0 + (t * s1_x), line_y0 + (t * s1_y)))
    }

    return None


}



fn parse_problem(reader: &mut BufReader<File>) -> (Vec<Vec<i64>>, Vec<Vec<i64>>){
    
    let mut positions = Vec::new();
    let mut velocities = Vec::new();

    for line in reader.lines(){

        let mut pos_vel = line.as_ref().unwrap().split(" @ ");

        let pos = pos_vel.next().unwrap().split(", ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let vel= pos_vel.next().unwrap().split(", ").map(|x| {println!("x: {}",x);x.parse::<i64>().unwrap()}).collect::<Vec<i64>>();


        positions.push(pos);
        velocities.push(vel);

    }

    return (positions, velocities)
}