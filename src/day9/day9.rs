use std::collections::HashSet;

const P1_KNOTS: usize = 2;
const P2_KNOTS: usize = 10;

pub fn day9_p1(input: &str) -> usize {
    day9(input, P1_KNOTS)
}

pub fn day9_p2(input: &str) -> usize {
    day9(input, P2_KNOTS)
}

fn day9(input: &str, num_knots: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); num_knots];
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for step in input.lines() {
        let mut step = step.split(' ');

        let dir = match step.next().unwrap() {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => (0, 0),
        };
        let count: usize = step.next().unwrap().parse().unwrap();

        for _ in 0..count {
            knots[0].0 += dir.0;
            knots[0].1 += dir.1;

            for knot in 1..num_knots {
                let dx = knots[knot - 1].0 - knots[knot].0;
                let dy = knots[knot - 1].1 - knots[knot].1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    knots[knot].0 += dx.signum();
                    knots[knot].1 += dy.signum();
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}
