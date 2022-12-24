use std::fs;
mod day3;

fn main() {
    let contents = fs::read_to_string("input/day3/input.txt").unwrap();
    println!("Day 3 part 1 solution: {}", day3::day3_p1(&contents));
    println!("Day 3 part 2 solution: {}", day3::day3_p2(&contents));
}

#[test]
fn test_day3_p1() {
    let contents = fs::read_to_string("input/day3/test.txt").unwrap();
    assert_eq!(day3::day3_p1(&contents), 157);
}

#[test]
fn test_day3_p2() {
    let contents = fs::read_to_string("input/day3/test.txt").unwrap();
    assert_eq!(day3::day3_p2(&contents), 70);
}
