use std::fs;
mod day4;

fn main() {
    let contents = fs::read_to_string("input/day4/input.txt").unwrap();
    println!("Day 4 part 1 solution: {}", day4::day4_p1(&contents));
    println!("Day 4 part 2 solution: {}", day4::day4_p2(&contents));
}

#[test]
fn test_day4_p1() {
    let contents = fs::read_to_string("input/day4/test.txt").unwrap();
    assert_eq!(day4::day4_p1(&contents), 2);
}

#[test]
fn test_day4_p2() {
    let contents = fs::read_to_string("input/day4/test.txt").unwrap();
    assert_eq!(day4::day4_p2(&contents), 4);
}
