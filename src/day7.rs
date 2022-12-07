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

    // fn children(&self) -> usize {
    //     self.subfolders
    //         .iter()
    //         .map(|f| {
    //             println!("children of {:?}", f.name);
    //             f.children()
    //         }).count()
    // }

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

    // fn find_folder(&self, name: String) -> Option<Folder> { // Retrieves parent folder name
    //     if self.subfolders.iter().any(|f| f.name == name) { // If folder is child of root
    //         return Some("/".to_owned())
    //     }

    //     let accumulated_str = self.find_parent_helper(name, "".to_owned());

    //     if !accumulated_str.is_empty() {
    //        Some(accumulated_str)
    //     }
    //     else {
    //         None
    //     }
    // }

    // fn find_parent_helper(&self, name: String, acc: String) -> String { 
    //     self.subfolders
    //         .iter()
    //         .map(|f| {
    //             if f.subfolders.iter().any(|child| child.name == name) {
    //                 acc.clone() + &f.name
    //             }
    //             else { 
    //                 f.find_parent_helper(name.clone(), acc.clone())
    //             }
    //         }).collect::<String>()
    // }


    fn add_subfolder(&mut self, location: String, folder: Folder) { // Adds a subfolder to the specified location
        if location == self.name { // base case
            self.subfolders.push(folder.clone())
        }

        for f in &mut self.subfolders { // rescursive case
            f.add_subfolder(location.clone(), folder.clone()) // implement copy ?
        }
    }

    fn find_parent(&self, name: String) -> Option<String> { // Retrieves parent folder name
        if self.subfolders.iter().any(|f| f.name == name) { // If folder is child of root
            return Some("/".to_owned())
        }

        let accumulated_str = self.find_parent_helper(name, "".to_owned());

        if !accumulated_str.is_empty() {
           Some(accumulated_str)
        }
        else {
            None
        }
    }

    // Auxilliary find parent function (recursive). Uses a string accumulator to keep track of status of folder.
    // (empty string = not found, otherwise string contains parent folder name).
    // Workaround for not being able to figure out how to implement a recursive function which returns Option<T> when all folders have been checked.
    // This means there's unecessary extra work involved in fetching the folder when we find it's parent (for backtracing with cd ..)
    fn find_parent_helper(&self, name: String, acc: String) -> String { 
        self.subfolders
            .iter()
            .map(|f| {
                if f.subfolders.iter().any(|child| child.name == name) {
                    acc.clone() + &f.name
                }
                else { 
                    f.find_parent_helper(name.clone(), acc.clone())
                }
            }).collect::<String>()
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
     
    let f1 = Folder::new("a".to_owned(), None, None);
    root.add_subfolder("/".to_owned(), f1);

    let f2 = Folder::new("b".to_owned(), None, None);
    root.add_subfolder("/".to_owned(), f2);

    let f3 = Folder::new("c".to_owned(), None, None);
    root.add_subfolder("b".to_owned(), f3);

    let f3 = Folder::new("d".to_owned(), None, None);
    root.add_subfolder("c".to_owned(), f3);

    println!("{:?}", root);

    // println!("{:?}", root.find_parent("s".to_owned()));
    // println!("{:?}", root.find_parent("d".to_owned()));
    // println!("{:?}", root.find_parent("a".to_owned()));

    println!("Fetching folder A ---- {:?}", root.fetch("a".to_owned()));



    println!("\n\n\n");

    data
    .split('\n')
    .for_each(|line| {
        if line.starts_with('$') {
            let cmd = line.split(' ').take(3).collect::<Vec<&str>>();
            // println!("{:?}", cmd);
            if cmd.len() > 2 && cmd[1] == "cd" {
                match cmd[2] { // dir name
                    "/" => {
                        current_folder = root.clone(); // inefficient
                        // println!("Arrived at root {}", current_folder.name);
                    }
                    ".." => {
                        // match root.find_parent(current_folder.name) {
                        //     Some(folder) => current_folder = root.fetch(folder),
                        //     None => current_folder = root.clone()
                        // }
                    }
                    _ => {
                        let new_folder = Folder::new(cmd[2].to_owned(), None, None); 
                        root.add_subfolder(current_folder.name.clone(), new_folder.clone());
                        current_folder = new_folder;
                    }
                }
            }
        }
    })
}


pub fn solve() {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    parse_commands(&input);
}

