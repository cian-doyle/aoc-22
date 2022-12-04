use day1::get_max_calories;
use day1::get_top_n_max_calories;
use day2::get_final_score;
use day3::get_priority_sum_part_two;
use day4::get_contains_pair_count;
use day4::get_overlapping_pair_count;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    // println!("{}", get_max_calories());
    // println!("{}", get_top_n_max_calories(3));
    // println!("{:?}", get_final_score());
    println!("{:?}", get_contains_pair_count());
    println!("{:?}", get_overlapping_pair_count());

}
