mod utils;
mod day1;

const SESSION_COOKIE: &str = "53616c7465645f5f550a96ca49a78e304d8621323b0ec4041cf3a7d5026e71a983c550d64b958c3c9ecf146201f0ff3b; session=53616c7465645f5fa7963d224672a5fa078e5f9acd3af16945247bdb62702e8adf7051b8d8d3ee83ea9d7054c97d0f034398171c41ec84ae739181134d654314";

fn main() {
    let data = utils::get_challenge_data(1);
    let solution = day1::calculate2(data);
    println!("{:?}", solution);
}
