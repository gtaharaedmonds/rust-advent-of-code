use std::fs;
mod day2;

fn main() {
    let contents = fs::read_to_string("input/day2/input.txt").unwrap();
    println!("Day 2 part 1 solution: {}", day2::day2_p1(&contents));
    println!("Day 2 part 2 solution: {}", day2::day2_p2(&contents));
}

#[test]
fn test_day2_p1() {
    let contents = fs::read_to_string("input/day2/test.txt").unwrap();
    assert_eq!(day2::day2_p1(&contents), 15);
}

#[test]
fn test_day2_p2() {
    let contents = fs::read_to_string("input/day2/test.txt").unwrap();
    assert_eq!(day2::day2_p2(&contents), 12);
}
