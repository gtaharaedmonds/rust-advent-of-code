use std::fs;
mod day8;

fn main() {
    let contents = fs::read_to_string("input/day8/input.txt").unwrap();
    println!("Day 8 part 1 solution: {}", day8::day8_p1(&contents));
    println!("Day 8 part 2 solution: {}", day8::day8_p2(&contents));
}

#[test]
fn test_day8_p1() {
    let contents = fs::read_to_string("input/day8/test.txt").unwrap();
    assert_eq!(day8::day8_p1(&contents), 21);
}

#[test]
fn test_day8_p2() {
    let contents = fs::read_to_string("input/day8/test.txt").unwrap();
    assert_eq!(day8::day8_p2(&contents), 8);
}
