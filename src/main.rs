mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod key;

fn main() {
    let data = utils::get_challenge_data(4);
    let solution1 = day4::solution1(&data);
    let solution2 = day4::solution2(&data);
    println!("solution 1: {:?}\nsolution 2: {:?}", solution1, solution2);
}
