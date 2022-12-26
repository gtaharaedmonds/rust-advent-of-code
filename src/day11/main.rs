use std::fs;
mod day11;

fn main() {
    let contents = fs::read_to_string("input/day11/input.txt").unwrap();
    println!("Day 11 part 1 solution: {}", day11::day11_p1(&contents));
    println!("Day 11 part 2 solution: {}", day11::day11_p2(&contents));
}

#[test]
fn test_day11_p1() {
    let contents = fs::read_to_string("input/day11/test.txt").unwrap();
    assert_eq!(day11::day11_p1(&contents), 10605);
}

#[test]
fn test_day11_p2() {
    let contents = fs::read_to_string("input/day11/test.txt").unwrap();
    assert_eq!(day11::day11_p2(&contents), 2713310158);
}
