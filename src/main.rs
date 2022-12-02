use day1::get_max_calories;
use day1::get_top_n_max_calories;
use day2::get_final_score;

mod day1;
mod day2;

fn main() {
    // println!("{}", get_max_calories());
    // println!("{}", get_top_n_max_calories(3));
    println!("{:?}", get_final_score());
}

// X = Loss
// Y = Draw 
// Z = Win