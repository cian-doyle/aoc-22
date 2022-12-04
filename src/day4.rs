use std::fs;


const PUZZLE_INPUT: &str = "data/pairs.txt";

struct Assignment {
    start: u32,
    finish: u32
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool { 
        self.start <= other.start && self.finish >= other.finish
    }
    fn overlapping(&self, other: &Assignment) -> bool { 
        (self.start >= other.start && self.start <= other.finish || self.finish >= other.start && self.finish <= other.finish) // first assignment overlaps
        ||
        (other.start >= self.start && other.start <= self.finish || other.finish >= self.start && other.finish <= self.finish) // second assignment overlaps
    }
}

impl TryFrom<&str> for Assignment {
    type Error = &'static str;

    fn try_from(range: &str) -> Result<Self, Self::Error> {
        match range.split_once("-") {
            Some((start, finish)) => Ok(Assignment{start: start.parse::<u32>().unwrap(), finish: finish.parse::<u32>().unwrap()}), // *
            None => Err("Invalid format"),
        }
    }
}

fn get_assignment_pairs(data: String) -> Vec<(Assignment, Assignment)> {
    data.split('\n').map(|line| {
        let (first, second) = line.split_once(",").unwrap(); 
        (Assignment::try_from(first).unwrap(), Assignment::try_from(second).unwrap()) // * todo error check parsing
    }).collect::<Vec<(Assignment, Assignment)>>()
}

pub fn get_contains_pair_count() -> u32 {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let assignment_pairs = get_assignment_pairs(input);

    let total_containing = assignment_pairs.iter().filter(|pair|
        pair.0.contains(&pair.1) || pair.1.contains(&pair.0)
    ).count();
    
    total_containing as u32
}

pub fn get_overlapping_pair_count() -> u32 {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let assignment_pairs = get_assignment_pairs(input);

    let total_overlapping = assignment_pairs.iter().filter(|pair|
        pair.0.overlapping(&pair.1)
    ).count();
 
    total_overlapping as u32
}