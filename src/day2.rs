
pub fn solution1 (data: &str) -> i32{
    let score: i32 = data.split("\n").map(|line| {
        let choices: Vec<&str> = line.split(" ").collect();
        match choices[0] {
            "A" => match choices[1] {
                "X" => return 4,
                "Y" => return 8,
                "Z" => return 3,
                _ => return 0,
            },
            "B" => match choices[1] {
                "X" => return 1,
                "Y" => return 5,
                "Z" => return 9,
                _ => return 0,
            },
            "C" => match choices[1] {
                "X" => return 7,
                "Y" => return 2,
                "Z" => return 6,
                _ => return 0,
            },
            _ => return 0,
        }
    }).sum();
    return score;
}

pub fn solution2 (data: &str) -> i32 {
    let score: i32 = data.split("\n").map(|line| {
        let choices: Vec<&str> = line.split(" ").collect();
        match choices[0] {
            "A" => match choices[1] {
                "X" => return 3,
                "Y" => return 4,
                "Z" => return 8,
                _ => return 0,
            },
            "B" => match choices[1] {
                "X" => return 1,
                "Y" => return 5,
                "Z" => return 9,
                _ => return 0,
            },
            "C" => match choices[1] {
                "X" => return 2,
                "Y" => return 6,
                "Z" => return 7,
                _ => return 0,
            },
            _ => return 0,
        }
    }).sum();
    return score;
}