use std::fs;
mod day6;

fn main() {
    let contents = fs::read_to_string("input/day6/input.txt").unwrap();
    println!("Day 6 part 1 solution: {}", day6::day6_p1(&contents));
    println!("Day 6 part 2 solution: {}", day6::day6_p2(&contents));
}

#[test]
fn test_day6_p1() {
    let contents: String = fs::read_to_string("input/day6/test.txt").unwrap();
    let mut lines = contents.lines();
    assert_eq!(day6::day6_p1(lines.next().unwrap()), 5);
    assert_eq!(day6::day6_p1(lines.next().unwrap()), 6);
    assert_eq!(day6::day6_p1(lines.next().unwrap()), 10);
    assert_eq!(day6::day6_p1(lines.next().unwrap()), 11);
}

#[test]
fn test_day6_p2() {
    let contents: String = fs::read_to_string("input/day6/test.txt").unwrap();
    let mut lines = contents.lines();
    assert_eq!(day6::day6_p2(lines.next().unwrap()), 23);
    assert_eq!(day6::day6_p2(lines.next().unwrap()), 23);
    assert_eq!(day6::day6_p2(lines.next().unwrap()), 29);
    assert_eq!(day6::day6_p2(lines.next().unwrap()), 26);
}
