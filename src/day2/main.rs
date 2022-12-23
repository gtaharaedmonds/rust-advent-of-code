use std::fs;
mod day2;

fn main() {
    let p1 = fs::read_to_string("input/day2/input.txt").unwrap();
    println!("Day 2 part 1 solution: {}", day2::day2_p1(&p1));
}

#[test]
fn test_day2_p1() {
    let contents = fs::read_to_string("input/day2/test.txt").unwrap();
    let result = day2::day2_p1(&contents);
    assert_eq!(result, 15);
}
