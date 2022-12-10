use std::{fs, collections::HashSet};

const PUZZLE_INPUT: &str = "data/rope.txt";

#[derive(Debug, Clone, Copy)]
enum Move {
    Up(isize),
    Down(isize),
    Right(isize),
    Left(isize)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize
}

#[derive(Debug, Clone, Copy)]
struct RopeState { 
    head: Position,
    tail: Position
}

impl RopeState {
    fn new(head: Position, tail: Position) -> RopeState {
        RopeState { head, tail }
    }

    fn touching(&self) -> bool { // if 2 knots are touching (diagonally or adjacent)
        (self.head.x - self.tail.x).abs() < 2 && (self.head.y - self.tail.y).abs() < 2
    }
}

fn parse_commands(data: &str) -> Vec<Move> {
    data
        .split('\n')
        .map(|line| {
            let cmd = line.split_once(' ').unwrap();
            match cmd.0 {
                "U" => Move::Up(cmd.1.parse::<isize>().unwrap()),
                "D" => Move::Down(cmd.1.parse::<isize>().unwrap()),
                "L" => Move::Left(cmd.1.parse::<isize>().unwrap()),
                "R" => Move::Right(cmd.1.parse::<isize>().unwrap()),
                _ => panic!("Unknown command")
            }
        }).collect::<Vec<Move>>()
}

fn move_rope(state: &mut Vec<RopeState>, visited_points: Option<&mut HashSet<Position>>, tail_movements: Option<&mut Vec<Move>>, step: Move) { 
    let latest_rope_pos = state.last().unwrap();
    let future_state: RopeState = match step {
        Move::Up(_) => {
            RopeState::new(Position { x: latest_rope_pos.head.x, y: latest_rope_pos.head.y + 1 }, latest_rope_pos.tail )
        },
        Move::Down(_) => {
            RopeState::new(Position { x: latest_rope_pos.head.x, y: latest_rope_pos.head.y - 1 }, latest_rope_pos.tail )
        },
        Move::Left(_) => {
            RopeState::new(Position { x: latest_rope_pos.head.x - 1, y: latest_rope_pos.head.y }, latest_rope_pos.tail )
        },
        Move::Right(_) => {
            RopeState::new(Position { x: latest_rope_pos.head.x + 1, y: latest_rope_pos.head.y }, latest_rope_pos.tail )
        }
    };
    if future_state.touching() { // If head is moving such that they are still directly adjacent or diagonal, then only move head
        state.push(future_state)
    }
    else { // Otherwise we need to move tail, which always moves to the last position of heads
        if let Some(v) = visited_points {
            v.insert(latest_rope_pos.head);
        }
        if let Some(t) = tail_movements {
            match step {
                Move::Up(_) => {
                   t.push(Move::Up(1))
                },
                Move::Down(_) => {
                    t.push(Move::Down(1))
                },
                Move::Left(_) => {
                    t.push(Move::Left(1))
                },
                Move::Right(_) => {
                    t.push(Move::Right(1))
                }
            }
        }
        state.push(RopeState::new(future_state.head, latest_rope_pos.head));
    }
}

// Part 1
fn count_visited_l(moves: &Vec<Move>, rope_length: isize) -> usize {

    let mut next_instructions = moves.clone();

    for _ in 0..rope_length-2 {
        let mut rope_positions = Vec::<RopeState>::new();
        rope_positions.push(RopeState::new(Position { x: 0, y: 0 }, Position { x: 0, y: 0 })); // Starting state
        let mut current_instructions = Vec::<Move>::new();

        next_instructions
            .iter()
            .for_each(|direction|
                match direction {
                    Move::Up(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, None, Some(&mut current_instructions), *direction)
                    }
                    Move::Down(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, None, Some(&mut current_instructions), *direction)
                    }
                    Move::Right(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, None, Some(&mut current_instructions), *direction)
                    }
                    Move::Left(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, None, Some(&mut current_instructions), *direction)
                    }            
                }
            );
        next_instructions = current_instructions;
    }

    let mut visited_points = HashSet::<Position>::new();
    visited_points.insert(Position { x: 0, y: 0 });
    let mut rope_positions = Vec::<RopeState>::new();
    rope_positions.push(RopeState::new(Position { x: 0, y: 0 }, Position { x: 0, y: 0 })); 

    if rope_length == 2 {
        next_instructions
            .iter()
            .for_each(|direction|
                match direction {
                    Move::Up(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction)
                    }
                    Move::Down(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction)
                    }
                    Move::Right(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction)
                    }
                    Move::Left(times) => for _ in 0..*times {
                        move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction)
                    }            
                }
            );
       
    }
    else {
        next_instructions
        .iter()
        .for_each(|direction|
            {
                match direction {
                    Move::Up(_) => move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction),
                    Move::Down(_) => move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction),
                    Move::Right(_) => move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction),
                    Move::Left(_) => move_rope(&mut rope_positions, Some(&mut visited_points), None, *direction)
                }
            }
           
        );
    }
    visited_points.len()
}

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let commands = parse_commands(&input);
    // println!("{:?}", commands);
    // println!("{:?}", count_visited(&commands));
    println!("{:?}", count_visited_l(&commands, 4));


    (0, 0)
}

