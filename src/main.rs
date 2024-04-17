use std::{
    env::args, fs, path::Path
};
use colored::{ColoredString, Colorize};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        println!("Too few argument: missing folder name");
        return;
    }
    else if args.len() > 2 {
        println!("Too many argument");
    }

    let folder = Path::new(&args[1]);

    match find_all(&folder) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    };
}

fn find_all(folder: &Path) -> Result<(), String> {
    if !folder.is_dir() {
        return Err("The path is not a diractory".into());
    }
    let mut dir_stack: Vec<Box<Path>> = Vec::new();
    let mut depth_stack: Vec<i32> = Vec::new();
    dir_stack.push(folder.into());
    depth_stack.push(0);

    while dir_stack.len() > 0 {
        let file_name = dir_stack.last().clone().unwrap();
        let depth = depth_stack.last().unwrap().clone();
        print_file(file_name, depth);
        depth_stack.pop();
        
        let dir = dir_stack.last().unwrap();
        let dir = fs::read_dir(dir);
        dir_stack.pop();
        let dir = match dir {
            Ok(d) => d,
            Err(_) => continue,
        };

        for entry in dir {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => break,
            };
            let path = entry.path();
            dir_stack.push(path.into());
            depth_stack.push(depth + 1);
        }
    }
    Ok(())
}

fn print_file(file_name: &Path, depth: i32) {
    let color: [ColoredString; 3] = ["-".bright_blue(), "-".red(), "-".green()];
    if depth > 0 { print!("{}", "|".yellow());}
    for i in 0..depth {
        let len = color.len() as i32;
        for _ in 0..4 {
            print!("{}", color[(i % len) as usize]);
        } 
    }
    if depth > 0 { print!(" ");}
    println!("{}", file_name
            .to_str().unwrap()
            .split('/')
            .last().unwrap());
}