use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_path = "src/inputs/day_05_input.txt";
    let (mut ship,instructions) = read_file(file_path);


    ship.pop();


    //Flip the arrays sideways
    //once flipped, we don't need the white spaces anymore

    let ship = flip(ship);

    let ship2 = ship.clone();
    let instructions2 = instructions.clone();

    
    let mut answer1 = move1(ship, instructions);
    let mut answer2 = move2(ship2, instructions2);
    
    let mut ans1: String = String::new();
    let mut ans2: String = String::new();

    for vec in answer1.iter_mut() {
        ans1.push(vec.pop().expect("empty"));
    }

    for vec in answer2.iter_mut() {
        ans2.push(vec.pop().expect("empty"));
    }

    println!("Part 1: {:#}", ans1);
    println!("Part 1: {:#}", ans2);


    


}
    

fn read_file(file_path: &str) -> (Vec<Vec<char>>,Vec<[usize;3]>) {

    let mut ship: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<[usize;3]> = Vec::new();
    
    //Read file and put it all in a vector of &str type.
    let mut file = File::open(file_path)
    .expect("file not found!");

    let mut data = String::new();
    
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    let v: Vec<&str> = data.split("\n").collect();

    let mut found = false;

    for line_str in v{
        
        
        if line_str == "" {
            found = true;
        }

        if found == false {
            let line_str: Vec<char> = line_str.chars().collect();

            let mut aux: Vec<char> = Vec::new();
            
            for i in (1..line_str.len()).step_by(4){
                aux.push(line_str[i]);
            }

            ship.push(aux);
            
        } else if line_str != "" {
            let line_str = line_str.replace("move ","")
            .replace(" from ",",")
            .replace(" to ",",");

            let line_str :Vec<&str> = line_str.split(",").collect();
            
            let mut aux: [usize;3] = [0;3];

            for i in 0..aux.len(){
                aux[i] = line_str[i].parse().unwrap();

            }

            instructions.push(aux);

        }
         

    }
    

    return (ship,instructions);
}

fn flip(ship: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let  mut flip: Vec<Vec<char>> = Vec::new();
    
    for i in 0..ship[0].len(){

        let mut aux: Vec<char> = Vec::new();

        for j in 0..ship.len(){
            if ship[j][i] != ' '{
                aux.insert(0, ship[j][i]);
            }
            
        }

        flip.push(aux);
    }

    return flip;

}


fn move1(mut ship: Vec<Vec<char>>,instructions: Vec<[usize;3]>) -> Vec<Vec<char>>{
    for instruction in instructions{ 
        let mut crane: Vec<char> = Vec::new();

        let num = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;


        //get boxes from stack
        for _i in 0..num{
            crane.push(ship[from].pop().expect("No item in this stack"));
        }

        //put boxes in other stack
        for _i in 0..num{
            ship[to].append(&mut crane);
        }
        
        
    }

    return ship.to_vec();

}

fn move2(mut ship: Vec<Vec<char>>,instructions: Vec<[usize;3]>) -> Vec<Vec<char>>{
    for instruction in instructions{ 
        let mut crane: Vec<char> = Vec::new();

        let num = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;


        //get boxes from stack
        for _i in 0..num{
            crane.push(ship[from].pop().expect("No item in this stack"));
        }

        //put boxes in other stack
        for _i in 0..num{
            ship[to].push(crane.pop().expect("empty"));
        }
        
    }

    return ship;

}