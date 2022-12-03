mod utils;
mod day1;
mod day2;
mod day3;
mod key;

fn main() {
    let data = utils::get_challenge_data(3);
    let solution1 = day3::solution1(&data);
    let solution2 = day3::solution2(&data);
    println!("solution 1: {:?}\nsolution 2: {:?}", solution1, solution2);
}
