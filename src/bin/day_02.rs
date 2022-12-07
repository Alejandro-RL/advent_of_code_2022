use std::fs::File;
use std::io::prelude::*;


fn main(){
    let file_path = "src/inputs/day_02_input.txt";
    let rounds: Vec<Vec<char>> = read_file(file_path);


    let mut total_score: i32 = 0;
    let mut total_score2: i32 = 0;

    
    for round in rounds{
        if round != [] {
            total_score += round_score_part1(&round);
            total_score2 += round_score_part2(&round);
        }        
    }
    

    println!(" Part 1: {}",total_score);
    println!(" Part 2: {}",total_score2);


}





fn read_file(file_path: &str) -> Vec<Vec<char>>{

    let mut rounds: Vec<Vec<char>> = Vec::new();
    
    

    let mut file = File::open(file_path)
    .expect("file not found!");

    let mut data = String::new();
    
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    let v: Vec<&str> = data.split("\n").collect();

    for line_str in v{
        let round: Vec<char> = line_str.chars().collect();
        rounds.push(round);
    }
 
   

    return rounds;
}

fn round_score_part1(round: &Vec<char>) -> i32 {
    let mut score: i32 = 0;

    let mut opponent: char = 'O';
    let mut me: char = 'O';


    //decode the plays to:
    // R - Rock
    // P - Paper
    // S - Scissors 
    match round[0] {
        'A' => opponent = 'R',
        'B' => opponent = 'P',
        'C' => opponent = 'S',
        _ => println!("error"),
    }

    match round[2] {
        'X' => me = 'R',
        'Y' => me = 'P',
        'Z' => me = 'S',
        _ => println!("error"),
    }


    //Conditions 

    if me == opponent{
        score += 3;
    }
    
    if me == 'R'{
        score += 1;
        if opponent == 'S' {
            score += 6;
        }

    } else if me == 'P'{
        score += 2;
        if opponent == 'R' {
            score += 6;
        }

    } else if me == 'S'{
        score += 3;
        if opponent == 'P' {
            score += 6;
        }

    }
    
    
    return score;


}

fn round_score_part2(round: &Vec<char>) -> i32 {
    let mut score: i32 = 0;

    let mut opponent: char = 'O';
    let me: char = round[2];


    //decode the plays to:
    // R - Rock
    // P - Paper
    // S - Scissors 
    match round[0] {
        'A' => opponent = 'R',
        'B' => opponent = 'P',
        'C' => opponent = 'S',
        _ => println!("error"),
    }




    //Conditions 
    // R = 1
    // P = 2
    // S = 3

    if me == 'Y' {
        score += 3;
    } else if me == 'Z'{
        score += 6;
    }

    if opponent == 'R' {
        if me == 'Y'{
            score += 1;
        } else if me == 'Z' {
            score += 2;
        } else if me == 'X' {
            score += 3;
        }
    } else if opponent == 'P' {
        if me == 'Y'{
            score += 2;
        } else if me == 'Z' {
            score += 3;
        } else if me == 'X' {
            score += 1;
        }
    } else if opponent == 'S' {
        if me == 'Y'{
            score += 3;
        } else if me == 'Z' {
            score += 1;
        } else if me == 'X' {
            score += 2;
        }
    }
    
    
    return score;


}
