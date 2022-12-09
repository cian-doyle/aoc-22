use std::{fs, ops::Sub, collections::HashSet};

const PUZZLE_INPUT: &str = "data/rope.txt";
type RopeStateHistory = Vec<RopeState>; // A record of a rope's movement

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
struct RopeState { // snapshot of lat
    head: Position,
    tail: Position
}

impl RopeState {
    fn new(head: Position, tail: Position) -> RopeState {
        RopeState { head, tail }
    }

    fn touching(&self) -> bool {
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

fn move_rope(state: &mut Vec<RopeState>, visited_points: &mut HashSet<Position>, tail_movements: &mut Vec<Move>, step: Move) {
    let latest_rope_pos = state.last().unwrap();
    // println!("Moving from {:?}", latest_rope_pos);
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
    if future_state.touching() { // If heads is moving such that they are still directly adjacent or diagonal, then only move head
        println!("Moving to {:?}", future_state);
        state.push(future_state)
    }
    else { // Otherwise we need to move tail, which always moves to the last position of heads
        println!("Moving to {:?}", RopeState::new(future_state.head, latest_rope_pos.head));
        visited_points.insert(latest_rope_pos.head);
        state.push(RopeState::new(future_state.head, latest_rope_pos.head));
    }
}

fn follow_knot(state: &mut Vec<RopeState>, target_position: Position) {
    let latest_rope_pos = state.last().unwrap();
    // println!("Moving from {:?}", latest_rope_pos);

    // is target_position and ltest state head are touching 
    let future_state = RopeState::new(target_position, latest_rope_pos.head);
        
    if future_state.touching() { // If heads is moving such that they are still directly adjacent or diagonal, then only move head
        println!("Moving to {:?}", future_state);
        state.push(future_state)
    }
    else { // Otherwise we need to move tail, which always moves to the last position of heads
        println!("Moving to {:?}", RopeState::new(future_state.head, latest_rope_pos.head));
        visited_points.insert(latest_rope_pos.head);
        state.push(RopeState::new(future_state.head, latest_rope_pos.head));
    }
}

// Part 1
fn count_visited(moves: &Vec<Move>) -> usize {
    let mut rope_positions = Vec::<RopeState>::new();
    let mut visited_points = HashSet::<Position>::new();
    visited_points.insert(Position { x: 0, y: 0 });

    rope_positions.push(RopeState::new(Position { x: 0, y: 0 }, Position { x: 0, y: 0 })); // Starting state
    moves
        .iter()
        .for_each(|direction|
            match direction {
                Move::Up(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }
                Move::Down(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }
                Move::Right(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }
                Move::Left(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }            
            }
        );
    visited_points.len()
}

// Part 1
fn count_visited_l(moves: &Vec<Move>, rope_length: usize) -> usize {
    let mut rope_positions = Vec::<RopeState>::new();
    let mut visited_points = HashSet::<Position>::new();
    visited_points.insert(Position { x: 0, y: 0 });

    rope_positions.push(RopeState::new(Position { x: 0, y: 0 }, Position { x: 0, y: 0 })); // Starting state
    let future_moves = 
    moves
        .iter()
        .for_each(|direction|
            match direction {
                Move::Up(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }
                Move::Down(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }
                Move::Right(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }
                Move::Left(times) => for _ in 0..*times {
                    move_rope(&mut rope_positions, &mut visited_points, *direction)
                }            
            }
        );
    visited_points.len()
}

// Part 2
fn count_visited_larger(moves: &Vec<Move>, rope_length: usize) -> usize {
    let mut rope_positions = Vec::<Vec<RopeState>>::new();
    let mut visited_points = HashSet::<Position>::new();
    visited_points.insert(Position { x: 0, y: 0 });

    let mut first_knot_movements = Vec::<RopeState>::new();// Record states for first knot pair 
    first_knot_movements.push(RopeState::new(Position { x: 0, y: 0 }, Position { x: 0, y: 0 })); // Starting state
    moves
        .iter()
        .for_each(|direction|
            match direction {
                Move::Up(times) => for _ in 0..*times {
                    move_rope(&mut first_knot_movements, &mut visited_points, *direction)
                }
                Move::Down(times) => for _ in 0..*times {
                    move_rope(&mut first_knot_movements, &mut visited_points, *direction)
                }
                Move::Right(times) => for _ in 0..*times {
                    move_rope(&mut first_knot_movements, &mut visited_points, *direction)
                }
                Move::Left(times) => for _ in 0..*times {
                    move_rope(&mut first_knot_movements, &mut visited_points, *direction)
                }            
            }
        );
    rope_positions.push(first_knot_movements);
    for knot in 0..rope_length {
        let mut knot_movements = Vec::<RopeState>::new(); 

        let recorded_tail_positions = rope_positions[knot] // follow tail of previous knot
            .iter()
            .map(|p|
                p.tail
            );

        for record in 0..recorded_tail_positions.len() {
            move_rope(&mut knot_movements, &mut visited_points, step)
        }
    }
    
    visited_points.len()
}


pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let commands = parse_commands(&input);
    println!("{:?}", commands);
    println!("{:?}", count_visited(&commands));

    (0, 0)
}

