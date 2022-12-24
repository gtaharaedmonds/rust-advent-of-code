use regex::Regex;
use std::cmp::{max, min};

pub fn day4_p1(input: &str) -> i32 {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut pairs: i32 = 0;

    for line in input.lines() { 
        let captures = re.captures(line).unwrap(); 
        let one = (captures[1].parse::<i32>().unwrap(), captures[2].parse::<i32>().unwrap()); 
        let two = (captures[3].parse::<i32>().unwrap(), captures[4].parse::<i32>().unwrap()); 

        if (one.0 <= two.0 && one.1 >= two.1) || (one.0 >= two.0 && one.1 <= two.1) {
            pairs += 1;
        }
    }

    pairs
}

pub fn day4_p2(input: &str) -> i32 {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut pairs: i32 = 0;

    for line in input.lines() { 
        let captures = re.captures(line).unwrap(); 
        let one = (captures[1].parse::<i32>().unwrap(), captures[2].parse::<i32>().unwrap()); 
        let two = (captures[3].parse::<i32>().unwrap(), captures[4].parse::<i32>().unwrap()); 

        if min(one.1, two.1) - max(one.0, two.0) >= 0 {
            pairs += 1;
        }
    }

    pairs
}