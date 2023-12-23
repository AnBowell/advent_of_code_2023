use std::{
    collections::{self, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    vec, ops::Range,
};

const FILE_LOC: &'static str = "data/test.txt";

fn main() {
    problem_one();

}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let mut lines = io::BufReader::new(file);


    let mut bricks = parse_problem_stack(&mut lines);

    bricks.sort_by(|a,b| a.z.1.cmp(&b.z.1));


    let mut blocks_to_remove = HashSet::new();

    for brick_counter in 0..bricks.len()-1{
        let current_brick = &bricks[brick_counter];

        if brick_counter + 1 >= bricks.len(){
            continue;
        }


        let next_brick = &bricks[brick_counter+1];

       let brick_pos =  next_brick.propagate_brick(&current_brick);

        println!("Brick pos: {:?}", brick_pos);


    }

    blocks_to_remove.insert(bricks.last().unwrap().no);




    
    // for x_counter in 0..block_tower.len(){
    //     println!("Block: {:?}", block_tower[0..block_tower.len()][x_counter][x_counter])
    // }
}


#[derive(Debug, Clone)]
struct Brick{
    no: usize,
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize)
}

impl Brick{
    fn propagate_brick(&self, brick_below: &Brick) -> Brick{

        if self.z == brick_below.z{
            return self.clone()
        }


        if self.x.0 >= brick_below.x.0 && self.x.1 <= brick_below.x.1 || self.y.0 >= brick_below.y.0 && self.y.1 <= brick_below.y.1{
            
            let mut brick_to_return = self.clone();
            brick_to_return.z = (brick_below.z.0 + 1, brick_below.z.1 + 1);
            return brick_to_return
        };

        let mut brick_to_return = self.clone();
        brick_to_return.z = (brick_below.z.0 + 1, brick_below.z.1 + 1);
        return brick_to_return
        

    }
}



fn parse_problem_stack(reader: &mut BufReader<File>) -> Vec<Brick>{

    let mut bricks = Vec::with_capacity(200);
    for (block_counter, line) in reader.lines().enumerate(){
        let mut start_end_loc = line.as_ref().unwrap().split("~");

        let start_coords = start_end_loc.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
   
        let end_coords = start_end_loc.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();




        let brick = Brick{ no: block_counter,  x: (start_coords[0], end_coords[0]),
             y:(start_coords[1], end_coords[1]), z: (start_coords[2], start_coords[2])};

        bricks.push(brick)

    }
    return bricks;
}


fn parse_problem(reader: &mut BufReader<File>)-> Vec<Vec<Vec<usize>>>{

    let size = 10;
    let mut block_tower = vec![vec![vec![20;10];3];3];

    for (block_counter, line) in reader.lines().enumerate(){
        let mut start_end_loc = line.as_ref().unwrap().split("~");

        let start_coords = start_end_loc.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
   
        let end_coords = start_end_loc.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();


        for x in start_coords[0]..=end_coords[0]{
            for y in start_coords[1]..=end_coords[1]{
                for z in start_coords[2]..=end_coords[2]{
                    block_tower[x][y][z] = block_counter;
                }
         
            }
            println!("block_tower: {:?}", block_tower[x])
        }
       
       
    }

    return block_tower


    
}