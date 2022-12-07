use std::fs;

const PUZZLE_INPUT: &str = "data/commands.txt";

// Assumes that we always start at root folder and don't unecessarily visit the same folder twice 
// (new folders are always instantiated when cd command encountered)
// Also assumes folders are one letter for simplicity 

#[derive(Clone, Debug)]
struct Folder {
    parent: Option<Box<Folder>>, 
    name: String,
    contents: Vec<File>, // could be option types
    subfolders: Vec<Folder>
}

impl Folder {
    fn new(parent: Option<Box<Folder>>, name: String, contents: Option<Vec<File>>, subfolders: Option<Vec<Folder>>) -> Self {
        match (contents, subfolders) {
            (None, None) => Folder { parent, name, contents: Vec::new(), subfolders: Vec::new() },
            (None, Some(subfolders)) => Folder { parent, name, contents: Vec::new(), subfolders },
            (Some(contents), None) => Folder { parent, name, contents, subfolders: Vec::new()},
            (Some(contents), Some(subfolders)) => Folder { parent, name, contents, subfolders },
        }
    }
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize
}

impl File {
    fn new(name: String, size: usize) -> Self {
        File { size, name }
    }
}

fn parse_commands(data: &str) {  
    let mut root = Folder::new(None, "/".to_owned(), None, None); // Assuming we always start from root directory?
    let mut current_folder: *mut Folder = &mut root;
     
    data
    .split('\n')
    .for_each(|line| {
        if line.starts_with('$') {
            let cmd = line.split(' ').take(3).collect::<Vec<&str>>();
            println!("{:?}", cmd);
            if cmd.len() > 2 && cmd[1] == "cd" {
                match cmd[2] { // dir name
                    "/" => {
                        println!("matched on /");
                        unsafe {
                            while (*current_folder).parent.is_some() {
                                println!("Going to root from {}", (*current_folder).name);
                                current_folder = &mut root;
                                println!("Arrived at root {}", (*current_folder).name);
                            } 
                        }
                    }
                    ".." => {
                        println!("matched on ..");
                        unsafe {
                            if (*current_folder).parent.is_some() {
                                println!("Changing from {}", (*current_folder).name);
                                current_folder = &mut *(*current_folder).parent.as_deref_mut().unwrap();
                                println!("To {}", (*current_folder).name);
                            }
                        }
                    }
                    _ => {
                        println!("matched on other");
                        unsafe {
                            println!("folder currently looks like {:?}", *current_folder);
                            (*current_folder).subfolders.push(Folder::new(Some(Box::new((*current_folder).clone())), cmd[2].to_owned(), None, None));
                            println!("made new folder, folder now looks like {:?}", *current_folder);
                        }
                    }
                }
            }
            // else {
            //     return panic!("Could not parse commands");
            // }
        }
    })
}


pub fn solve() {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    parse_commands(&input);
}

