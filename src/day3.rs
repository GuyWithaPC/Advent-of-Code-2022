use std::slice;
static PRIORITIES: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                                 'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

pub fn solution1(data: &str) -> i32 {
    let priority_sum = data.split('\n').map(|input| {
        let size = input.len();
        let (first_half, second_half) = input.split_at(size / 2);
        let (mut first_half, mut second_half): (Vec<char>, Vec<char>) = (first_half.chars().collect(), second_half.chars().collect());
        first_half.dedup();
        second_half.dedup();
        let mut answer: char = 'a';
        'outer: for character_a in &first_half {
            for character_b in &second_half {
                if character_a == character_b {
                    answer = *character_a;
                    break 'outer
                }
            }
        };
        return get_priority(answer)
    }).sum();
    return priority_sum;
}

pub fn solution2(data: &str) -> i32 {
    let data_strings: Vec<&str> = data.split('\n').collect();
    let priority_sum: i32 = data_strings.chunks_exact(3).into_iter().map(|chunk| {
        let counts: Vec<(bool,bool,bool)> = PRIORITIES.iter().map(|letter| {
            println!("{:?}",chunk);
            let mut sum = (false,false,false);
            for letter_b in chunk[0].chars() {
                if letter_b == *letter {
                    sum.0 = true;
                }
            }
            for letter_b in chunk[1].chars() {
                if letter_b == *letter {
                    sum.1 = true;
                }
            }
            for letter_b in chunk[2].chars() {
                if letter_b == *letter {
                    sum.2 = true;
                }
            }
            return sum
        }).collect();
        let mut priority = 0;
        for num in counts.iter().enumerate() {
            if num.1.0 == true && num.1.1 == true && num.1.2 == true {
                priority = num.0 as i32 + 1;
            }
        }
        return priority
    }).sum();
    return priority_sum;
}

fn get_priority(character: char) -> i32 {
    PRIORITIES.iter().position(|&x| x == character).unwrap() as i32 + 1
}