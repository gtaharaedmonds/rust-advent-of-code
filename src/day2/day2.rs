use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

fn op_throw(throw: &char) -> Throw {
    HashMap::from([
        ('A', Throw::Rock),
        ('B', Throw::Paper),
        ('C', Throw::Scissors),
    ])
    .get(throw)
    .unwrap()
    .to_owned()
}

fn my_throw(throw: &char) -> Throw {
    HashMap::from([
        ('X', Throw::Rock),
        ('Y', Throw::Paper),
        ('Z', Throw::Scissors),
    ])
    .get(throw)
    .unwrap()
    .to_owned()
}

fn throw_score(throw: &Throw) -> i32 {
    HashMap::from([(Throw::Rock, 1), (Throw::Paper, 2), (Throw::Scissors, 3)])
        .get(throw)
        .unwrap()
        .to_owned()
}

fn score_throws(me: &Throw, op: &Throw) -> i32 {
    let score_map = HashSet::from([
        (&Throw::Rock, &Throw::Scissors),
        (&Throw::Paper, &Throw::Rock),
        (&Throw::Scissors, &Throw::Paper),
    ]);

    if score_map.contains(&(me, op)) {
        // Win
        6
    } else if score_map.contains(&(op, me)) {
        // Loss
        0
    } else {
        // Draw
        3
    }
}

pub fn day2_p1(input: &str) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        let op = op_throw(&line_vec[0]);
        let me = my_throw(&line_vec[2]);
        score += score_throws(&me, &op) + throw_score(&me);
    }

    score
}
