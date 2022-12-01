use day1::get_max_calories;

use crate::day1::get_top_n_max_calories;
mod day1;

fn main() {
    println!("{}", get_max_calories());
    println!("{}", get_top_n_max_calories(3));
}