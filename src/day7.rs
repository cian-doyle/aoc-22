
use std::{fs};

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
            })
            .filter(|res| res.is_some())
            .last()
            .unwrap_or(None) // No results in whole tree, assumes root as 
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

        if self.subfolders.iter().any(|f| f.name == folder.name) { // duplicate ?
            return
        }

        for f in &mut self.subfolders { // Recursive case, target folder not found yet
            f.add_folder(location.clone(), folder.clone()); // implement copy ?
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

    fn content_size(&self) -> usize {  // Gets sum of files only, not including those in subfolders
        self.contents.iter().map(|f| f.size).sum::<usize>()
    }

    fn sum_folders_inclusive(&self) -> usize {  // Gets total size of a folder, including files in subfolders

        let mut state = 0;

        sum_state(self, &mut state);

        fn sum_state(folder: &Folder, state: &mut usize) {
            *state += folder.content_size();

            for node in &folder.subfolders { // Repeat for children
                sum_state(node, state)
            }
        }

        state
    }

    fn get_total_sums(&self, limit: usize) -> usize {  // Get size of all folders under limit, including recounts

        let mut state = Vec::<(Folder, usize)>::new();

        sum_folder(self, &mut state);

        fn sum_folder(tree: &Folder, state: &mut Vec::<(Folder, usize)>) {
            let inclusive_size = tree.sum_folders_inclusive();

            state.push((tree.clone(), inclusive_size)); // Add folders inclusive size to vec
            
            for node in &tree.subfolders { // Repeat for children
                sum_folder(node, state)
            }
        }

        state
            .into_iter()
            .filter(|(_f, size)| size <= &limit)
            .fold(0, |acc, (folder, size)| acc + size)
    }
   
}

fn build_folder_tree(data: &str) -> Folder { // Builds a hierarchy of nested folders and file 
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
                tree.add_folder(current_folder.clone().name, Folder::new(cmd[1].to_owned(), None, None));
            }
            else if let Ok(file_size) = cmd[0].parse::<usize>() { // New file
                tree.add_file(current_folder.clone().name, File::new(file_size, cmd[1].to_owned()))
            }
        });

    tree
}

fn parse_commands(data: &str) {  
    let mut tree = build_folder_tree(data);

    println!("Total of tree folders inclusive {:#?}", tree.get_total_sums(100000));
}


pub fn solve() {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    parse_commands(&input);
}

// Resulting struct from build_tree()
// Folder {
//     name: "/",
//     contents: [
//         File {
//             size: 14848514,
//             name: "b.txt",
//         },
//         File {
//             size: 8504156,
//             name: "c.dat",
//         },
//     ],
//     subfolders: [
//         Folder {
//             name: "a",
//             contents: [
//                 File {
//                     size: 29116,
//                     name: "f",
//                 },
//                 File {
//                     size: 2557,
//                     name: "g",
//                 },
//                 File {
//                     size: 62596,
//                     name: "h.lst",
//                 },
//             ],
//             subfolders: [
//                 Folder {
//                     name: "e",
//                     contents: [
//                         File {
//                             size: 584,
//                             name: "i",
//                         },
//                     ],
//                     subfolders: [],
//                 },
//             ],
//         },
//         Folder {
//             name: "d",
//             contents: [
//                 File {
//                     size: 4060174,
//                     name: "j",
//                 },
//                 File {
//                     size: 8033020,
//                     name: "d.log",
//                 },
//                 File {
//                     size: 5626152,
//                     name: "d.ext",
//                 },
//                 File {
//                     size: 7214296,
//                     name: "k",
//                 },
//             ],
//             subfolders: [],
//         },
//     ],
// }
