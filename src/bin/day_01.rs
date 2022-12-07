use std::fs::File;
use std::io::prelude::*;


fn main(){
    let file_path = "src/inputs/day_01_input.txt";
    read_file(file_path);
}


fn read_file(file_path: &str){

    let mut elves: Vec<i32> = Vec::new();
    
    

    let mut file = File::open(file_path)
    .expect("file not found!");

    let mut data = String::new();
    
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    let v: Vec<&str> = data.split("\n").collect();

    let mut cal_sum: i32 = 0;

    for cal in v{
        

        if cal != "" {
            let cal_int : i32  = cal.parse().unwrap();
            cal_sum += cal_int;

        } else {
            elves.push(cal_sum);
            cal_sum = 0;

        }
    }
  
    let maxvalue = elves.iter().max();
    println!("{:?}", maxvalue);

    elves.sort();
    elves.reverse();

    let mut maxvalues: i32 = 0;

    for i in 0..3{ 
        maxvalues += elves[i];
    }

    println!("{}",maxvalues);
}
