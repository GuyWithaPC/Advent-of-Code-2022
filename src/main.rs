mod utils;
mod day1;
mod day2;
mod key;

fn main() {
    let data = utils::get_challenge_data(2);
    let solution1 = day2::solution1(&data);
    let solution2 = day2::solution2(&data);
    println!("solution 1: {:?}\nsolution 2: {:?}", solution1, solution2);
}
