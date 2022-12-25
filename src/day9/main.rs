use std::fs;
mod day9;

fn main() {
    let contents = fs::read_to_string("input/day9/input.txt").unwrap();
    println!("Day 9 part 1 solution: {}", day9::day9_p1(&contents));
    println!("Day 9 part 2 solution: {}", day9::day9_p2(&contents));
}

#[test]
fn test_day9_p1() {
    let contents = fs::read_to_string("input/day9/test.txt").unwrap();
    assert_eq!(day9::day9_p1(&contents), 13);
}

#[test]
fn test_day9_p2() {
    let test1 = fs::read_to_string("input/day9/test.txt").unwrap();
    let test2 = fs::read_to_string("input/day9/test_p2_bigger.txt").unwrap();
    assert_eq!(day9::day9_p2(&test1), 1);
    assert_eq!(day9::day9_p2(&test2), 36);
}
