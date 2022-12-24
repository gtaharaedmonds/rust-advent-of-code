use std::fs;
mod day7;

fn main() {
    let contents = fs::read_to_string("input/day7/input.txt").unwrap();
    println!("Day 7 part 1 solution: {}", day7::day7_p1(&contents));
    println!("Day 7 part 2 solution: {}", day7::day7_p2(&contents));
}

#[test]
fn test_day7_p1() {
    let contents = fs::read_to_string("input/day7/test.txt").unwrap();
    assert_eq!(day7::day7_p1(&contents), 95437);
}

#[test]
fn test_day7_p2() {
    let contents = fs::read_to_string("input/day7/test.txt").unwrap();
    assert_eq!(day7::day7_p2(&contents), 24933642);
}
