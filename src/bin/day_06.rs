fn main() {
    let datastream = include_str!("../inputs/day_06_input.txt");
    let answer1: u32 = parse(&datastream,4);
    let answer2: u32 = parse(&datastream,14);

    println!("Part 1: {}",answer1);
    println!("Part 2: {}",answer2);
}

fn parse(datastream: &&str, size: usize) -> u32{
    let mut char_pos: u32 = 0;
    let str: Vec<char> = datastream.chars().collect();
    let mut check: String = " ".to_string();

    for char in str{

        if check.len() == (size + 1) && unique(&check){
            break;
        }
        
        if !unique(&check){
            check.remove(1);
        }
        check.push(char);
        char_pos += 1;
        
    } 

    println!("{:?}",check);
    return char_pos;

}

fn unique(check: &String) -> bool{
    let mut answer: bool = true;
    let compare: Vec<char> = check.chars().collect();

    for i in 0..(compare.len() - 1){
        for j in (i+1)..compare.len() {
            if compare[i] == compare[j]{
                answer = false;
            }
        } 
    }
    return  answer;
}