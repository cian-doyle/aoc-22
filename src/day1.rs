use std::fs;

pub fn get_max_calories(path: &str) -> u32 {
    let calories = fs::read_to_string(path).expect("Can't read file");
    let elf_vec = calories.split("\n\n").collect::<Vec<&str>>();

    let mut total_calories_vec = Vec::<u32>::new();

    for elf in elf_vec {
        let sum = elf.split("\n").map(|x| x.parse::<u32>().unwrap()).sum::<u32>();
        total_calories_vec.push(sum);
    }

    total_calories_vec.iter().fold(0, |a, &b| a.max(b))
}