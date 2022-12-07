use core::{fmt};
use std::fs;

const PUZZLE_INPUT: &str = "data/commands.txt";

#[derive(Clone, Debug)]
struct File {
    size: usize,
    name: String
}

impl File {
    fn new(size: usize, name: String) -> Self {
        File { size, name }
    }
}

#[derive(Clone, Debug)]
struct Folder { 
    name: String,
    contents: Vec<File>, 
    subfolders: Vec<Folder>
}

impl Folder {
    fn new(name: String, contents: Option<Vec<File>>, subfolders: Option<Vec<Folder>>) -> Self {
        match (contents, subfolders) { // Probably not needed with current implementation
            (None, None) => Folder { name, contents: Vec::new(), subfolders: Vec::new() },
            (None, Some(subfolders)) => Folder { name, contents: Vec::new(), subfolders },
            (Some(contents), None) => Folder { name, contents, subfolders: Vec::new()},
            (Some(contents), Some(subfolders)) => Folder { name, contents, subfolders }, 
        }
    }

    fn fetch(&self, name: String) -> Option<Folder> { // Recursive fetch
        self.subfolders
            .iter()
            .map(|f| {
                if f.name == name { // Base case, parent of target folder found
                    Some(f.clone())
                }
                else { // Recursive case, not found yet
                    f.fetch(name.clone()) 
                }
            }).filter(|res| res.is_some()).last().unwrap_or(None) // No results in whole tree
    }

    fn find_parent(&self, name: String) -> Option<Folder> { // Recursive search
        self.subfolders
            .iter()
            .map(|f| {
                if f.subfolders.iter().any(|child| child.name == name) { // Base case, parent of target folder found
                    Some(f.clone())
                }
                else { 
                    f.find_parent(name.clone()) // Recursive case, not found yet
                }
            }).filter(|res| res.is_some()).last().unwrap_or(None) // No results in whole tree
    }

    fn add_folder(&mut self, location: String, folder: Folder) { // Adds a subfolder to the specified location (folder name as string) (Recursive)
        if location == self.name { // Base case, target folder to add new subfolder in found
            self.subfolders.push(folder.clone())
        }

        for f in &mut self.subfolders { // Recursive case, target folder not found yet
            f.add_folder(location.clone(), folder.clone()) // implement copy ?
        }
    }

    fn add_file(&mut self, location: String, file: File)  {  // Adds a file to the specified location (folder name as string) (Recursive)
        if location == self.name { // Base case, target folder to add new file in found
            self.contents.push(file.clone())
        }

        for f in &mut self.subfolders { // Recursive case, target folder not found yet
            f.add_file(location.clone(), file.clone())
        }
    }

    fn sum_folders(&mut self) -> usize {  // Adds a file to the specified location (folder name as string) (Recursive)
        if self.subfolders.is_empty() {
            return self.contents.iter().map(|f| f.size).sum::<usize>()
        }
        let sum = self.subfolders
            .iter_mut()
            .map(|f| {
                if !f.contents.is_empty() {
                    f.contents.iter().map(|f| f.size).sum::<usize>()
                }
                else { 
                    f.sum_folder()
                }
            }).sum::<usize>();
        sum
    }
}

fn build_folder_tree(data: &str) -> Folder {
    let mut tree = Folder::new("/".to_owned(), None, None); // Assuming we always start from root directory?
    let mut current_folder = tree.clone();

    data
        .split('\n')
        .for_each(|line| {
            let cmd = line.split(' ').take(3).collect::<Vec<&str>>();
            if cmd.len() > 2 && cmd[1] == "cd" {
                match cmd[2] { // folder name/arg
                    "/" => { // Go to root
                        current_folder = tree.clone(); 
                    }
                    ".." => { // move up 1 folder
                        if let Some(parent) = tree.find_parent(current_folder.clone().name) {
                            current_folder = parent;                        
                        }
                    }
                    _ => { // move down 1 folder
                        if let Some(target) = tree.fetch(cmd[2].to_owned()) {
                            current_folder = target;
                        }
                    }
                }
            }
            else if cmd[0] == "dir" { // New subfolder
                tree.add_folder(current_folder.clone().name,  Folder::new(cmd[1].to_owned(), None, None));
            }
            else if let Ok(file_size) = cmd[0].parse::<usize>() {
                tree.add_file(current_folder.clone().name, File::new(file_size, cmd[1].to_owned()))
            }
        });

    tree
}

fn parse_commands(data: &str) {  
    let mut tree = build_folder_tree(data);

    println!("{:#?}", tree);

    println!("Size of e  {:#?}", tree.fetch("e".to_string()).unwrap().sum_folder());

    println!("Size of /    {:#?}", tree.sum_folder());


    // Folder { 
    //     name: "/", 
    //     contents: [], 
    //     subfolders: [
    //         Folder { 
    //             name: "a", 
    //             contents: [], 
    //             subfolders: [
    //                 Folder { 
    //                     name: "e", 
    //                    contents: [], 
    //                     subfolders: [] 
    //                 }
    //             ] 
    //         }, 
    //         Folder { 
    //             name: "d", 
    //             contents: [], 
    //             subfolders: [] 
    //         }
    //     ] 
    // }
}


pub fn solve() {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    parse_commands(&input);
}

