fn main() {
    let datastream = include_str!("../inputs/day_08_input.txt");
    let trees = parse_file(datastream);
    let (answer1,answer2) = visible_trees(&trees);

    println!("Part 1: {:?}",answer1);
    println!("Part 2: {:?}",answer2);

}

fn parse_file(datastream: &str) -> Vec<Vec<u32>> {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    let lines: Vec<&str> = datastream.split("\n").collect();

    for line in lines{
        let mut  vec: Vec<&str> = line.split("").collect();
        vec.remove(0);
        vec.pop();      
        trees.push(vec.iter().map(|x| x.parse::<u32>().unwrap() ).collect()); 
    }
    trees
}


fn visible_trees(trees: &Vec<Vec<u32>>) -> (usize,u32) {
    let mut tree_count:usize = 0;
    let mut max_scene_score:u32 = 0;

    for i in 0..trees.len(){
        for j in 0..trees[0].len(){
            let (visible,score) = is_visible(i, j, trees);
            if visible{
                tree_count += 1;
            }
            if score > max_scene_score{
                max_scene_score = score;
            }
        }
    }

    (tree_count,max_scene_score)
}

fn is_visible(start_i: usize, start_j:usize, trees: &Vec<Vec<u32>>) -> (bool,u32) {
    let mut  visible: u32 = 4;
    let mut scenes: [u32;4] = [0;4];
    let len_i = trees.len();
    let len_j = trees[0].len();
    



    //look bottom
    for i in start_i+1..len_i{
        scenes[0] += 1;
        if trees[i][start_j] >= trees[start_i][start_j] {
            visible -= 1;
            break;
        }
    }

    //look top
    for i in (0..start_i).rev(){
        scenes[1] += 1;
        if trees[i][start_j] >= trees[start_i][start_j] {
            visible -= 1;
            break;
        }
    }

    //look right
    for j in start_j+1..len_j{
        scenes[2] += 1;
        if trees[start_i][j] >= trees[start_i][start_j] {
            visible -= 1;
            break;
        }
    }

    //look left
    for j in (0..start_j).rev(){
        scenes[3] += 1;
        if trees[start_i][j] >= trees[start_i][start_j] {
            visible -= 1;
            break;
        }
    }


    let mut score: u32 = 1;
    for scene in scenes{
        score *= scene;
    }



    if visible == 0 {
        return (false,score);
    } else {
        return (true,score);
    }
}
