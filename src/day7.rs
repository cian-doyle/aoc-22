use core::fmt;
use std::fs;

const PUZZLE_INPUT: &str = "data/commands.txt";

// Assumes that we always start at root folder and don't unecessarily visit the same folder twice 
// (new folders are always instantiated when cd command encountered)

#[derive(Clone)]
struct Folder { // should probably have a parent struct
    name: String,
    contents: Vec<File>, 
    subfolders: Vec<Folder>
}

impl fmt::Debug for Folder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}/", self.name)
        // if self.subfolders.is_empty() {
        //     write!(f, "\n{}/", self.name)
        // }
        // else {
            write!(f, "\n{}/ -> {:?}", self.name, self.subfolders)
        // }
    }
}

impl std::cmp::PartialEq for Folder {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Folder {
    fn new(name: String, contents: Option<Vec<File>>, subfolders: Option<Vec<Folder>>) -> Self {
        match (contents, subfolders) {
            (None, None) => Folder { name, contents: Vec::new(), subfolders: Vec::new() },
            (None, Some(subfolders)) => Folder { name, contents: Vec::new(), subfolders },
            (Some(contents), None) => Folder { name, contents, subfolders: Vec::new()},
            (Some(contents), Some(subfolders)) => Folder { name, contents, subfolders },
        }
    }

    fn fetch(&self, name: String) -> Option<Folder> { 
        let folder = self.subfolders
            .iter()
            .map(|f| {
                if f.name == name {
                    Some(f.clone())
                }
                else {
                    f.fetch(name.clone())
                }
            }).filter(|res| res.is_some()).last();

        match folder {
            Some(folder) => folder,
            None => None
        }
    }

    fn find_parent(&self, name: String) -> Option<Folder> { 
        let folder = self.subfolders
            .iter()
            .map(|f| {
                if f.subfolders.iter().any(|child| child.name == name) {
                    Some(f.clone())
                }
                else { 
                    f.find_parent(name.clone())
                }
            }).filter(|res| res.is_some()).last();

            match folder {
                Some(folder) => folder,
                None => None
            }
    }

    fn add_subfolder(&mut self, location: String, folder: Folder) { // Adds a subfolder to the specified location
        if location == self.name { // base case
            self.subfolders.push(folder.clone())
        }

        for f in &mut self.subfolders { // rescursive case
            f.add_subfolder(location.clone(), folder.clone()) // implement copy ?
        }
    }

    fn add_file(&mut self, file: File) { 
        self.contents.push(file)
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
    let mut root = Folder::new("/".to_owned(), None, None); // Assuming we always start from root directory?
    let mut current_folder = root.clone();
     
    // let f1 = Folder::new("a".to_owned(), None, None);
    // root.add_subfolder("/".to_owned(), f1);

    // let f2 = Folder::new("b".to_owned(), None, None);
    // root.add_subfolder("/".to_owned(), f2);

    data
    .split('\n')
    .for_each(|line| {
        if line.starts_with('$') {

            let cmd = line.split(' ').take(3).collect::<Vec<&str>>();
            println!("{:?}", cmd);

            if cmd.len() > 2 && cmd[1] == "cd" {
                match cmd[2] { // dir name
                    "/" => {
                        current_folder = root.clone(); 
                        println!("Going to root dir");
                    }
                    ".." => {
                        println!("Going up from {}", current_folder.name);
                        match root.find_parent(current_folder.clone().name) {
                            Some(parent) => {
                                current_folder = parent;
                                println!("Arrived at {}", current_folder.name);
                            }
                            None => println!("Already at root folder!")
                        }
                    }
                    "ls" => {
                        println!("Going up from {}", current_folder.name);
                        match root.find_parent(current_folder.clone().name) {
                            Some(parent) => {
                                current_folder = parent;
                                println!("Arrived at {}", current_folder.name);
                            }
                            None => println!("Already at root folder!")
                        }
                    }
                    _ => todo!()
                }
            }
        }
    })
}


pub fn solve() {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    parse_commands(&input);
}

