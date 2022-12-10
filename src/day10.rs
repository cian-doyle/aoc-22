use std::{collections::VecDeque, fs};
const PUZZLE_INPUT: &str = "data/cpu_instructions.txt";

#[derive(Debug)]
struct CPU { 
    register: Register,
    instruction_queue: VecDeque<Instruction>, 
    clock: isize,
    screen: String // maybe rename struct
}

#[derive(Debug)]
enum InstructionType { 
    NOP,
    ADD
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    value: Option<isize>,
    cycles_left: isize,
}

impl Instruction {
    fn new(instruction_type: InstructionType, value: Option<isize>, cycles_left: isize) -> Instruction {
        Instruction { instruction_type, value, cycles_left }
    }
}

#[derive(Clone, Copy, Debug)]
struct Register {
    name: char,
    value: isize
}

impl Register {
    fn new(name: char) -> Register {
        Register { name, value: 1 }
    }
}

impl CPU {
    fn new(register: Register, instruction_queue: VecDeque::<Instruction>) -> CPU {
        CPU { register, instruction_queue, clock: 1, screen: "".to_string() }
    }

    fn update_register(&mut self, value: isize) {
        let current_instruction = self.get_latest_instruction();
        if current_instruction.cycles_left == 1 {
            self.register.value = value;
            self.instruction_queue.pop_front();
        }
        else {
            current_instruction.cycles_left -= 1;
        }
    } 

    fn draw_pixel(&mut self) {
        let pixel = self.clock % 40;
        if pixel.abs_diff(self.register.value) <= 1 {
            self.screen.push('#');
        }
        else {
            self.screen.push('.')
        }
        if pixel == 39 { 
            self.screen.push('\n')
        }
    }

    fn get_latest_instruction(&mut self) -> &mut Instruction {
        self.instruction_queue.get_mut(0).unwrap()
    }

    fn cycle(&mut self) { 
        let latest_instruction =  self.instruction_queue.front().unwrap();       
        match latest_instruction {
            Instruction { instruction_type: InstructionType::ADD, value, .. } => {
                self.update_register(self.register.value + value.unwrap());
            }
            Instruction { instruction_type: InstructionType::NOP, .. } => {
                let latest_instruction = self.get_latest_instruction(); //todo refactor duplicate logic
                latest_instruction.cycles_left -= 1;
                if latest_instruction.cycles_left == 0 {
                    self.instruction_queue.pop_front();
                }
            },
            _ => panic!("Invalid command"), 
        }
        self.draw_pixel();
        self.clock += 1;
    }
}

fn cycles_needed(instruction: InstructionType) -> isize {
    match instruction {
        InstructionType::ADD => 2,
        InstructionType::NOP => 1
    } 
}

fn parse_instructions(data: &str) -> CPU {
    let mut instruction_queue = VecDeque::<Instruction>::new();
    data
        .split('\n')
        .for_each(|line| {
            let line_split = line.split_once(' ');
            match line_split {
                Some((instruction, value)) => {
                    // let register_name = instruction.chars().nth(3);
                    let instr = instruction.chars().take(3).collect::<String>();
                    let instruction_str = instr.as_str();
                    match instruction_str { 
                        "add" => {
                            instruction_queue.push_back(Instruction::new(InstructionType::ADD, value.parse::<isize>().ok(), cycles_needed(InstructionType::ADD)))
                        },
                        _ => {
                            panic!("Unknown instruction encountered")
                        }
                    }
                }
                None => instruction_queue.push_back(Instruction::new(InstructionType::NOP, None, cycles_needed(InstructionType::NOP)))
            }
        });
        CPU::new(Register::new('x'), instruction_queue)
}

pub fn solve() -> isize {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let mut cpu = parse_instructions(&input);
    let mut total_signal_strength = 0;

    for _ in 0..240 {
        cpu.cycle();
        if cpu.clock == 20 || (cpu.clock - 20) % 40 == 0  {
            total_signal_strength += cpu.clock * cpu.register.value
        }
    }

    println!("{}", cpu.screen);
    total_signal_strength
}

