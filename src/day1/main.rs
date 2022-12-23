use std::fs;
mod day1;

fn main() {
    let contents = fs::read_to_string("input/day1/input.txt").unwrap();

    let p1 = day1::day1_p1(&contents);
    println!("Day 1 part 1 solution: {}", p1);

    let p2 = day1::day1_p2(&contents);
    println!("Day 1 part 1 solution: {}", p2);
}

#[test]
fn test_day1_p1() {
    let contents = fs::read_to_string("input/day1/test.txt").unwrap();
    let result = day1::day1_p1(&contents);
    assert_eq!(result, 24000);
}

#[test]
fn test_day1_p2() {
    let contents = fs::read_to_string("input/day1/test.txt").unwrap();
    let result = day1::day1_p2(&contents);
    assert_eq!(result, 45000);
}
