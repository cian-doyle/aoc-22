use std::{fs, collections::VecDeque};

const PUZZLE_INPUT: &str = "data/crates.txt";

type CrateStack = VecDeque<char>;
type Instruction = Vec<u32>;


fn parse_lines(data: &str) -> Vec<Vec<char>> { // Builds vec [ [' ', 'D', ' '], ['N', 'C', ' '] ] ... etc
    data.split('\n')
        .map(|line| {
            let line_chars = line.chars().collect::<Vec<char>>();
            line_chars 
                .chunks(4) // cut into 4 char chunks and condense chunks to single char, with ' ' meaning no crate
                .map(|chunk|
                    if chunk[1].is_whitespace() { 
                        ' '
                    }
                    else { 
                        chunk[1] 
                    } 
                )
                .collect::<Vec<char>>()
        })
        .take_while(|vec| !vec[0].is_numeric())
        .collect()
}

fn get_crate_stacks(data: &str) -> Vec<CrateStack> { // build vector of stacks(which are VecDequeue<char>). Chars popped from back of vector = removed from top of stack (visually)
    let parsed_lines = parse_lines(data);
    let mut stacks = Vec::<CrateStack>::new();

    for _ in 0 .. parsed_lines[0].len() { 
        stacks.push(CrateStack::new())
    }

    parsed_lines // populate 'stacks' vector with crates, using index in vec to assign to correct stack
        .iter()
        .for_each(|line|
            for index in 0 .. line.len() {
                if !line[index].is_whitespace() {
                    stacks[index].push_front(line[index])
                }
            }
        );
    stacks
}

fn parse_instructions(data: &str) -> Vec<Instruction> { // "move 1 from 3 to 2"     ->      "[1, 3, 2]"
    data.split('\n')
        .filter(|line| line.starts_with('m'))
        .map(|instruction_line| 
            instruction_line
                .split(' ')
                .filter_map(|slice| slice.parse::<u32>().ok())
                .collect::<Instruction>()
        ).collect::<Vec<Instruction>>()
}

// multiple crate mover 
fn execute_instructions_cm9001(stacks: &mut [CrateStack], instructions: Vec<Instruction>) { 
    instructions.iter().for_each(|instruction_data| { 
        let amount = instruction_data[0] as usize;
        let from = instruction_data[1] as usize;
        let to = instruction_data[2] as usize;

        let mut temp_crate_vec = VecDeque::<char>::with_capacity(amount);

        for _ in 0 .. amount { // Push crates to intermediate VecDequeue before pushing them to requested stack
            if let Some(moved_crate) = stacks[from - 1].pop_back() {
                temp_crate_vec.push_front(moved_crate);
            }
        }

        for _ in 0 .. amount { // pop crates onto requested stack from front
            if let Some(moved_crate) = temp_crate_vec.pop_front() {
                stacks[to - 1].push_back(moved_crate);
            }
        }
    });
}

fn get_top_crates_str(stacks: &[CrateStack]) -> String {
    stacks
        .iter()
        .filter_map(|stack| stack.back())
        .collect()
}

pub fn solve() -> String {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let mut crate_stacks = get_crate_stacks(&input);
    let instructions = parse_instructions(&input);
    // execute_instructions_cm9000(&mut crate_stacks, instructions);
    execute_instructions_cm9001(&mut crate_stacks, instructions);
    get_top_crates_str(&crate_stacks)
}

// single crate mover
// fn execute_instructions_cm9000(stacks: &mut [CrateStack], instructions: Vec<Instruction>) {
//     instructions.iter().for_each(|instruction_data| {  // * make instruction struct instead of type alias ?
//         let amount = instruction_data[0];
//         let from = instruction_data[1] as usize;
//         let to = instruction_data[2] as usize;

//         for _ in 0 .. amount {
//             if let Some(moved_crate) = stacks[from - 1].pop_back() {
//                 stacks[to - 1].push_back(moved_crate);
//             }
//         }
//     });
// }
