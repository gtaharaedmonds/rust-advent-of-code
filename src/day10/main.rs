use std::fs;
mod day10;

fn main() {
    let contents = fs::read_to_string("input/day10/input.txt").unwrap();
    println!("Day 10 part 1 solution: {}", day10::day10_p1(&contents));
    println!("Day 10 part 2 solution:\n{}", day10::day10_p2(&contents));
}

#[test]
fn test_day10_p1() {
    let contents = fs::read_to_string("input/day10/test.txt").unwrap();
    assert_eq!(day10::day10_p1(&contents), 13140);
}

#[test]
fn test_day10_p2() {
    let contents = fs::read_to_string("input/day10/test.txt").unwrap();
    println!("{}", day10::day10_p2(&contents));
    // Not quite, but close!
}
