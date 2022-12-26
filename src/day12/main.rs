use std::fs;
mod day12;

fn main() {
    let contents = fs::read_to_string("input/day12/input.txt").unwrap();
    println!("Day 12 part 1 solution: {}", day12::day12_p1(&contents));
    println!("Day 12 part 2 solution: {}", day12::day12_p2(&contents));
}

#[test]
fn test_day12_p1() {
    let contents = fs::read_to_string("input/day12/test.txt").unwrap();
    assert_eq!(day12::day12_p1(&contents), 31);
}

#[test]
fn test_day12_p2() {
    let contents = fs::read_to_string("input/day12/test.txt").unwrap();
    assert_eq!(day12::day12_p2(&contents), 29);
}
