use std::{fs::{self}, io::BufRead};

const PUZZLE_INPUT: &str = "data/calories.txt";

// Part 1
pub fn get_max_calories() -> u32 {
    let total_calories_vec: Vec<u32> = get_elf_calorie_counts();
    total_calories_vec.iter().fold(0, |a, &b| a.max(b))
   
}

// Part 2
pub fn get_top_n_max_calories(n: usize) -> u32 {
    let mut total_calories_vec: Vec<u32> = get_elf_calorie_counts();
    total_calories_vec.sort();
    total_calories_vec.drain(total_calories_vec.len() - n ..).sum()
}

fn get_elf_calorie_counts() -> Vec<u32> {
    let calories = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let elf_vec = calories.split("\n\n").collect::<Vec<&str>>();

    let mut total_calories_vec = Vec::<u32>::new();

    for elf in elf_vec {
        let sum = elf.split("\n").map(|x| x.parse::<u32>().unwrap()).sum::<u32>();
        total_calories_vec.push(sum);
    }

    total_calories_vec
}



