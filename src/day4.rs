use std::ops::Range;
use itertools::Itertools;

pub fn solution1 (data: &str) -> i32 {
    let containers: i32 = data.split("\n").map(|pair| {
        if pair == "" {
            return 0
        }
        let (first_range, second_range): (&str, &str) = pair.split_once(",").unwrap();
        let (first_range,second_range): ((i32,i32),(i32,i32)) = (
            {
                let first_range_vec: Vec<i32> = first_range.split("-").map(
                    |item| item.parse::<i32>().unwrap()
                ).collect();
                (*first_range_vec.get(0).unwrap(),*first_range_vec.get(1).unwrap())
            },
            {
                let second_range_vec: Vec<i32> = second_range.split("-").map(
                    |item| item.parse::<i32>().unwrap()
                ).collect();
                (*second_range_vec.get(0).unwrap(),*second_range_vec.get(1).unwrap())
            }
        );
        if first_range.0 >= second_range.0 {
            if first_range.1 <= second_range.1 {
                return 1
            }
        }
        if second_range.0 >= first_range.0 {
            if second_range.1 <= first_range.1 {
                return 1
            }
        }
        println!("({:?}, {:?}",first_range,second_range);
        return 0
    }).sum();
    return containers
}

pub fn solution2 (data: &str) -> i32 {
    let overlappers: i32 = data.split("\n").map(|pair| {
        if pair == "" {
            return 0
        }
        let (first_range, second_range): (&str, &str) = pair.split_once(",").unwrap();
        let (first_range,second_range): ((i32,i32),(i32,i32)) = (
            {
                let first_range_vec: Vec<i32> = first_range.split("-").map(
                    |item| item.parse::<i32>().unwrap()
                ).collect();
                (*first_range_vec.get(0).unwrap(),*first_range_vec.get(1).unwrap())
            },
            {
                let second_range_vec: Vec<i32> = second_range.split("-").map(
                    |item| item.parse::<i32>().unwrap()
                ).collect();
                (*second_range_vec.get(0).unwrap(),*second_range_vec.get(1).unwrap())
            }
        );
        if first_range.0 <= second_range.1 && second_range.0 <= first_range.1 {
            return 1
        }
        println!("({:?}, {:?}",first_range,second_range);
        return 0
    }).sum();
    return overlappers
}