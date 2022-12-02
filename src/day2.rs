use std::{fs, str::FromStr};

const PUZZLE_INPUT: &str = "data/rps.txt";

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6
}

struct Round {
    opponents_shape: Shape,
    desired_outcome: Outcome
}

impl Round {
    fn resolve_score(&self) -> u32 {
        let shape_score = match self.opponents_shape {
            Shape::Rock => {
                match self.desired_outcome {
                    Outcome::Lose => Shape::Scissors as u32,
                    Outcome::Draw => Shape::Rock as u32,
                    Outcome::Win => Shape::Paper as u32
                }
            }
            Shape::Paper => {
                match self.desired_outcome {
                    Outcome::Lose => Shape::Rock as u32,
                    Outcome::Draw => Shape::Paper as u32,
                    Outcome::Win => Shape::Scissors as u32
                }
            }
            Shape::Scissors => {
                match self.desired_outcome {
                    Outcome::Lose => Shape::Paper as u32,
                    Outcome::Draw => Shape::Scissors as u32,
                    Outcome::Win => Shape::Rock as u32
                }
            },
        };
    
        shape_score + (self.desired_outcome as u32)
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        if let Some((opp_shape, outcome)) = s.split_once(' ') {
            let opponents_shape = match opp_shape {
                "A" => Some(Shape::Rock), 
                "B" => Some(Shape::Paper), 
                "C" => Some(Shape::Scissors),
                _ => None
            };

            let desired_outcome = match outcome {
                "X" => Some(Outcome::Lose), 
                "Y" => Some(Outcome::Draw), 
                "Z" => Some(Outcome::Win),  
                _ => None
            };

            if opponents_shape.is_none() || desired_outcome.is_none() { return Err(()) }

            Ok(Round { opponents_shape: opponents_shape.unwrap(), desired_outcome: desired_outcome.unwrap() })
        }
        else  { Err(()) }
    }
}

pub fn get_final_score() -> u32 {
    let game_data = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let rounds = game_data.split("\n").collect::<Vec<&str>>();
    let score = rounds.iter().map(|line| Round::from_str(line).unwrap().resolve_score()).sum::<u32>(); 
    score
}

// fn resolve_match_points(round: &str) -> u32 {
//     match round {
//         "A X" => 4,
//         "A Y" => 8,        
//         "A Z" => 3,        
//         "B X" => 1,        
//         "B Y" => 5,        
//         "B Z" => 9,        
//         "C X" => 7,        
//         "C Y" => 2,        
//         "C Z" => 6,        
//         _ => 0
//     }
// }
