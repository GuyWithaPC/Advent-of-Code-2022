mod utils;
mod day1;
mod key;

fn main() {
    let data = utils::get_challenge_data(1);
    let solution = day1::calculate2(data);
    println!("{:?}", solution);
}
