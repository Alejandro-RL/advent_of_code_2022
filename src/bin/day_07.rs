struct Folder{
    name: String,
    file_size: u64,
    parent: String,
    children: Vec<String>,
}

fn build_folder(name: String) -> Folder {
    Folder {
        name,
        file_size: 0,
        parent: String::new(),
        children: Vec::new(),
    }

}

/*
fn print_folders(folders: &Vec<Folder>){
    //debugging 
    for folder in folders{
        println!("----------------------------");
        println!("{:?}",folder.name);
        println!("{:?}",folder.file_size);
        println!("{:?}",folder.parent);
        println!("{:?}",folder.children);
    }

}
*/

fn main() {
    let datastream = include_str!("../inputs/day_07_input.txt");
    let cmds: Vec<&str> = datastream.split("\n").collect();
    let folders = map_folders(&cmds);
    let answer1: u64 = total_sum(&folders);
    let answer2: u64 = delete(&folders);
    
    
    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}

fn map_folders<'a>(cmds: &'a Vec<&'a str>) -> Vec<Folder>{
    // the file system will represented as a vector of folders
    let mut folders: Vec<Folder> = Vec::new();

    // the order in which folders are accessed
    let mut order: Vec<&str> = Vec::new();

    // length of the folders vector
    let mut len: usize = 0;

    for cmd in cmds{
        // each command in the list of commands
        let cmd: Vec<&str> = cmd.split(" ").collect();

        if cmd.contains(&".."){
            //moves out one level
            order.pop();
            
        }else if cmd.contains(&"cd") {
            //change directory to cmd[2]
            //there's only one cd cmd per folder

            let parent = order.join("/");

            order.push(cmd[2]);
            let name =  order.join("/");
            folders.push(build_folder(name));

        
            len += 1;


            // root folder already starts at 0 index
            if cmd[2] != "/" {
                //This folder has a parent
                folders[len - 1].parent = parent;
            }
            

        }else if cmd.contains(&"dir") {
            //folders that are children of level folder
            let mut c = order.clone();
            c.push(cmd[1]);
            folders[len - 1].children.push(c.join("/").to_string());

        }else if cmd[0].parse::<u64>().is_ok() {
            //if it starts with a number
            //add to the last folder file size
            folders[len - 1].file_size += cmd[0].parse::<u64>().unwrap();
        }
    }


    //Folders don't have the size of their subfolders
    //to add that:
    
    for i in (0..folders.len()).rev(){
        //if the folder has a parent
        if folders[i].parent != "" {
            //find the parent
            for j in (0..folders.len()).rev(){
                if folders[j].name == folders[i].parent {
                    // add the file_size
                    folders[j].file_size += folders[i].file_size;
                    // a folder always has one parent, so we don't need to keep searching
                    break;
                }
            }
        }

    }
    

    folders
}

fn total_sum(folders: &Vec<Folder>) -> u64 {
    let mut sum: u64 = 0;

    for folder in folders {
        if folder.file_size <= 100000 {
            sum += folder.file_size;
        }
    }
    sum
}

fn delete(folders: &Vec<Folder>) -> u64 {
    let size = 30000000 - (70000000 - folders[0].file_size);
    let mut possible_answers: Vec<u64> = Vec::new();

    for folder in folders{
        if folder.file_size >= size {
            possible_answers.push(folder.file_size);
        }
    }

    return *possible_answers.iter().min().unwrap();

}