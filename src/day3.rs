use std::{fs, collections::HashSet};

const PUZZLE_INPUT: &str = "data/backpacks.txt";

#[derive(Debug)]
struct Rucksack {
    first_component: String,
    second_component: String
}

impl TryFrom<(&str, &str)> for Rucksack {
    type Error = &'static str;

    fn try_from(components: (&str, &str)) -> Result<Self, Self::Error> {
        if components.0.len() != components.1.len() {
            return Err("Backpack compartments are not equal")
        }
        Ok(Rucksack{first_component: components.0.to_string(), second_component: components.1.to_string()})
        
    }
}

fn find_duplicate_items(data: String) -> Vec<char> {
    let rucksack_vec = data.split("\n").map(|line| 
        Rucksack::try_from(line.split_at(line.len()/2)).unwrap() // todo error handling
    ).collect::<Vec<Rucksack>>();

    rucksack_vec.iter().map(|rs| { 
        let mut seen_chars = HashSet::<char>::new();
        rs.first_component.chars().for_each(|c| { 
            seen_chars.insert(c);
        });
        rs.second_component.chars().find(|c|
            seen_chars.contains(c)
        ).unwrap() // todo error handling, assumes duplicate char is in both components

    }).collect::<Vec<char>>()
}

fn char_to_priority(c: &char) -> u32 { // using ascii codes
    if c.is_lowercase() { 
        *c as u32 - 96 
    }
    else { 
        *c as u32 - 38 
    }
}


pub fn get_priority_sum() -> u32 {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let duplicate_items = find_duplicate_items(input);

    //println!("Duplicate chars: {:?}", duplicate_items);

    duplicate_items.iter().map(char_to_priority).sum::<u32>()
}