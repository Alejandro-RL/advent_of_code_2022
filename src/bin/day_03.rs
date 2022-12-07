use std::fs::File;
use std::io::prelude::*;


fn main(){
    let file_path = "src/inputs/day_03_input.txt";
    let rucksacks: Vec<Vec<char>> = read_file(file_path);
    let rucksacks2 = rucksacks.clone();

    //items that appear in both compartiments
    let mut items: Vec<char> = Vec::new();
    let mut items2: Vec<char> = Vec::new();

    for rucksack in rucksacks{
        if rucksack == []{
            break;
        }
        items.push(find_item_part1(&rucksack));
    }

    //items that appear between three elves
    for i in (0..rucksacks2.len()-2).step_by(3){
        items2.push(find_item_part2([&rucksacks2[i],&rucksacks2[i+1],&rucksacks2[i+2]]));

    }

   

    //time to sum priorities
    let mut priority_sum1: u32 = 0;
    let mut priority_sum2: u32 = 0;


    //a - 97
    //A - 65
    for item in items{
        if item.is_lowercase() {
            priority_sum1 += (item as u32) - 96;
            
        } else {
            priority_sum1 += (item as u32) - 38;
        }
    }

    for item in items2{
        if item.is_lowercase() {
            priority_sum2 += (item as u32) - 96;
            
        } else {
            priority_sum2 += (item as u32) - 38;
        }
    }
     

    println!("Part 1: {}",priority_sum1);
    println!("Part 2: {}",priority_sum2);


}





fn read_file(file_path: &str) -> Vec<Vec<char>>{

    let mut rucksacks: Vec<Vec<char>> = Vec::new();
    
    let mut file = File::open(file_path)
    .expect("file not found!");

    let mut data = String::new();
    
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    let v: Vec<&str> = data.split("\n").collect();

    for line_str in v{
        let rucksack: Vec<char> = line_str.chars().collect();
        rucksacks.push(rucksack);
    }
 
    return rucksacks;
}

fn find_item_part1(rucksack: &Vec<char>) -> char{
    let length = rucksack.len();
    let mut item: char = '0';
    
    for i in 0..(length/2){
        for j in (length/2)..length{
            if rucksack[i] == rucksack[j] {
                //println!("The found item is {}",rucksack[i]);
                item = rucksack[i];
            }
        }
        
    }

    return item;
}

fn find_item_part2(rucksacks: [&Vec<char> ; 3]) -> char{
    let mut item: char = '0';
    
    for i in 0..rucksacks[0].len() {
        for j in 0..rucksacks[1].len() {
            for k in 0..rucksacks[2].len() {
                if rucksacks[0][i] == rucksacks[1][j] && rucksacks[1][j] == rucksacks[2][k]{
                    item = rucksacks[0][i];
                } 
        
            }
        }
    }


    return item;
}  
