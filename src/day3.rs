use std::{fs, collections::{HashSet, HashMap}};

const PUZZLE_INPUT: &str = "data/backpacks.txt";
const CHUNK_SIZE: usize = 3;

struct Rucksack {
    first_component: String,
    second_component: String
}

impl Rucksack {
    fn unique_char_str(&self) -> String { // trim duplicates and concat both components into one string
        let mut unique_chars = HashSet::<char>::new();
        self.first_component.chars().for_each(|c| { 
            unique_chars.insert(c);
        });
        self.second_component.chars().for_each(|c| { 
            unique_chars.insert(c);
        });
        unique_chars.into_iter().collect()
    }
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

fn find_duplicate_items(data: String) -> Vec<char> { // Find commmon char in both components of a rucksack
    let rucksack_vec = get_rucksacks(data);

    rucksack_vec.iter().map(|rs| { 
        let mut seen_chars = HashSet::<char>::new();
        rs.first_component.chars().for_each(|c| { 
            seen_chars.insert(c);
        });
        rs.second_component.chars().find(|c|
            seen_chars.contains(c)
        ).unwrap() // todo error handling

    }).collect::<Vec<char>>()
}

fn char_to_priority(c: &char) -> u32 { // Converted using ascii codes
    if c.is_lowercase() { 
        *c as u32 - 96 
    }
    else { 
        *c as u32 - 38 
    }
}

fn get_rucksacks(data: String) -> Vec<Rucksack> {
    data.split('\n').map(|line| 
        Rucksack::try_from(line.split_at(line.len()/2)).unwrap() // todo error handling
    ).collect::<Vec<Rucksack>>()
}

pub fn get_priority_sum_part_one() -> u32 {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let duplicate_items = find_duplicate_items(input);

    duplicate_items.iter().map(char_to_priority).sum::<u32>()
}

pub fn get_priority_sum_part_two() -> u32 {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let rucksack_vec = get_rucksacks(input);

    let elf_groups = rucksack_vec.chunks(CHUNK_SIZE);

    // Strings for each groups aggregated rucksacks with duplicate chars removed
    let group_rucksacks = elf_groups.map(|group| {
        group.iter().map(|rucksack| {
            rucksack.unique_char_str()
        }).collect::<String>()
    }).collect::<Vec<String>>();

    // Search each group rucksack for char with 3 occurences and get priority char
    let mut group_badge_chars = Vec::<char>::new();

    group_rucksacks.iter().for_each(|aggregate_rucksack| {
        let mut char_occurences = HashMap::<char, u32>::new();
        aggregate_rucksack.chars().for_each(|c| {
            char_occurences.entry(c).and_modify(|counter| *counter += 1).or_insert(1);      
            if *char_occurences.get(&c).unwrap() == 3 {
                group_badge_chars.push(c);
            }
        });
    });

    // Get sum of priorities
    group_badge_chars.iter().map(
        char_to_priority
    ).sum::<u32>()
}