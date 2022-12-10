use std::{collections::VecDeque, fs};
const PUZZLE_INPUT: &str = "data/cpu_instructions.txt";

#[derive(Debug)]
struct CPU { 
    registers: Vec<Register>,
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
    register_used: Option<char>,
    value: Option<isize>,
    cycles_left: isize,
}

impl Instruction {
    fn new(instruction_type: InstructionType, register_used: Option<char>, value: Option<isize>, cycles_left: isize) -> Instruction {
        Instruction { instruction_type, register_used, value, cycles_left }
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
    fn new(registers: Vec::<Register>, instruction_queue: VecDeque::<Instruction>) -> CPU {
        CPU { registers, instruction_queue, clock: 1, screen: "".to_string() }
    }

    fn update_register(&mut self, name: char, value: isize) {
        let current_instruction = self.get_latest_instruction();
        if current_instruction.cycles_left == 1 {
            let register = self.registers.iter_mut().find(|r| r.name == name).unwrap();
            register.value = value;
            self.instruction_queue.pop_front();
        }
        else {
            current_instruction.cycles_left -= 1;
        }
    } 

    fn draw_pixel(&mut self) {
        let pixel = self.clock % 40;
        if pixel.abs_diff(self.registers.first().unwrap().value) <= 1 {
            self.screen.push('#');
        }
        else {
            self.screen.push('.')
        }
        if pixel == 39 { 
            self.screen.push('\n')
        }
    }

    fn get_register_value(&self, name: char) -> isize { 
        self.registers.iter().find(|r| r.name == name).unwrap().value
    }

    fn get_latest_instruction(&mut self) -> &mut Instruction {
        self.instruction_queue.get_mut(0).unwrap()
    }

    fn cycle(&mut self) { 
        let latest_instruction =  self.instruction_queue.front().unwrap();       
        match latest_instruction {
            Instruction { instruction_type: InstructionType::ADD, register_used, value, .. } => {
                let register_name = register_used.unwrap();
                let current_register_value = self.get_register_value(register_name);
                self.update_register(register_name, current_register_value + value.unwrap());
            }
            Instruction { instruction_type: InstructionType::NOP, .. } => {
                let latest_instruction = self.get_latest_instruction();
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
                    let register_name = instruction.chars().nth(3);
                    let instr = instruction.chars().take(3).collect::<String>();
                    let instruction_str = instr.as_str();
                    match instruction_str { // wouldnt work if instruction set has instructions that are not 3 chars besides noop
                        "add" => {
                            instruction_queue.push_back(Instruction::new(InstructionType::ADD, register_name, value.parse::<isize>().ok(), cycles_needed(InstructionType::ADD)))
                        }, // could add more instructions here
                        _ => {
                            panic!("Unknown instruction encountered")
                        }
                    }
                }
                None => instruction_queue.push_back(Instruction::new(InstructionType::NOP, None, None, cycles_needed(InstructionType::NOP)))
            }
        });
        let registers = vec![Register::new('x')]; // todo add more registers
        CPU::new(registers, instruction_queue)
}

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let mut cpu = parse_instructions(&input);
    let mut total_signal_strength = 0;

    for i in 0..240 {
        cpu.cycle();
        if cpu.clock == 20 || (cpu.clock - 20) % 40 == 0  {
            total_signal_strength += cpu.clock * cpu.registers.first().unwrap().value
        }
    }

    println!("{}", cpu.screen);
    (0, 0)
}

