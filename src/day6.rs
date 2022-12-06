use std::{fs, collections::{VecDeque, HashSet}};

const PUZZLE_INPUT: &str = "data/stream.txt";

struct Buffer<T> {
    contents: VecDeque<T>,
    size: usize,
    counter: usize,
    unique_set: HashSet<T>
}

impl<T: std::cmp::Eq + std::hash::Hash + Copy> Buffer<T> {
    fn push(&mut self, value: &T) { // Push newest char to back and pop oldest from front if buffer is fully sized
        if self.contents.len() == self.size { 
            let removed_value = self.contents.pop_front().unwrap();
            if !self.contents.contains(&removed_value) { // remove char from hashset if char is not anywhere else in buffer
                self.unique_set.remove(&removed_value);
            } 
        }
        self.contents.push_back(*value);
        self.unique_set.insert(*value);
        self.counter += 1;
    }

    fn fully_unique(&self) -> bool { // If buffer is full and hashset is same size of buffer, marker has been found
        self.unique_set.len() == self.size && self.unique_set.len() == self.contents.len()
    }

    fn new(size: usize) -> Buffer<T> { 
        Buffer { contents: VecDeque::new(), size, counter: 0, unique_set: HashSet::new() }
    }
}

fn find_marker(data: &str, marker_size: usize) -> Option<usize> {
    let mut buffer = Buffer::<char>::new(marker_size);
    for char in data.chars() {
        buffer.push(&char);    
        if buffer.fully_unique() {
            return Some(buffer.counter)
        }
    }            
    None
}

pub fn solve() -> usize {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    //find_marker(&input, 4)
    find_marker(&input, 14).unwrap()
}

