use std::fs;
mod day5;

fn main() {
    let contents = fs::read_to_string("input/day5/input.txt").unwrap();
    println!("Day 5 part 1 solution: {}", day5::day5_p1(&contents));
    println!("Day 5 part 2 solution: {}", day5::day5_p2(&contents));
}

#[test]
fn test_day5_p1() {
    let contents = fs::read_to_string("input/day5/test.txt").unwrap();
    assert_eq!(day5::day5_p1(&contents), "CMZ");
}

#[test]
fn test_day5_p2() {
    let contents = fs::read_to_string("input/day5/test.txt").unwrap();
    assert_eq!(day5::day5_p2(&contents), "MCD");
}
