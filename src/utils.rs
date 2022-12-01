
use crate::SESSION_COOKIE;
use ureq;
use std::fs::{self,File};
use std::io::{Write,Read};
use std::path::Path;

pub fn get_challenge_data(day: usize) -> String {
    let file_string = format!("inputcache/day{}.txt",day);
    let file_path = Path::new(&file_string);
    let mut data = String::new();
    if file_path.exists() {
        println!("reading data from cache...");
        let mut buf = Vec::new();
        File::open(file_path)
            .expect("Failed to open cached data file!")
            .read_to_end(&mut buf).expect("Failed to read cached data file!");
        data = String::from_utf8(buf).expect("Failed to parse cached data file!");

    } else {
        println!("cache not found! performing GET request...");
        data = ureq::get(&format!("https://adventofcode.com/2021/day/{}/input", day))
            .set("cookie", &format!("ru={}", SESSION_COOKIE))
            .call().expect("HTTP GET request failed")
            .into_string().expect("Failed to grab a string");
        File::create(file_path)
            .expect("Unable to create cache file!")
            .write_all(data.as_bytes())
            .expect("Unable to write to cache file!");
    }
    return data
}