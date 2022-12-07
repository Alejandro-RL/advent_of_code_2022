use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_path = "src/inputs/day_04_input.txt";
    let pairs = read_file(file_path);
    let mut counter: u32 = 0;

    //Verify if contains.

    for pair in &pairs{
        if (pair[0] <= pair[2] && pair[1] >= pair[3]) ||
        (pair[2] <= pair[0] && pair[3] >= pair[1]){
            counter += 1;
        }
    }
    println!("Part 1: {}",counter);

    counter = 0;

    for pair in &pairs{
        if (pair[1] >= pair[2] && pair[1] <= pair[3]) ||
        (pair[3] >= pair[0] && pair[3] <= pair[1]){
            println!("{:?}",&pair);
            counter += 1;
        }
    }

    println!("Part 2: {}",counter);
    //Verifies if it overlaps at all.



}

fn read_file(file_path: &str) -> Vec<[u32;4]>{

    let mut pairs: Vec<[u32;4]> = Vec::new();
    
    
    let mut file = File::open(file_path)
    .expect("file not found!");

    let mut data = String::new();
    
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    let v: Vec<&str> = data.split("\n").collect();

    for line_str in v{
        let pair_str: Vec<&str> = line_str.split(&['-',','][..]).collect();
        let mut pair: [u32;4] = [0;4];
        for i in 0..pair.len(){
            pair[i] = pair_str[i].parse().unwrap();
        }
        pairs.push(pair);
        
    }
    return pairs;
}
