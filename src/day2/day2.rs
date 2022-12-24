use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

fn decode_op_throw(throw: &char) -> Throw {
    match throw {
        'A' => Some(Throw::Rock),
        'B' => Some(Throw::Paper),
        'C' => Some(Throw::Scissors),
        _ => None,
    }
    .unwrap()
}

fn decode_my_throw(throw: &char) -> Throw {
    match throw {
        'X' => Some(Throw::Rock),
        'Y' => Some(Throw::Paper),
        'Z' => Some(Throw::Scissors),
        _ => None,
    }
    .unwrap()
}

fn game_result(me: &Throw, op: &Throw) -> GameResult {
    let results = HashSet::from([
        (&Throw::Rock, &Throw::Scissors),
        (&Throw::Paper, &Throw::Rock),
        (&Throw::Scissors, &Throw::Paper),
    ]);

    if results.contains(&(me, op)) {
        GameResult::Win
    } else if results.contains(&(op, me)) {
        GameResult::Loss
    } else {
        GameResult::Draw
    }
}

fn score_result(result: &GameResult) -> i32 {
    match result {
        GameResult::Win => 6,
        GameResult::Loss => 0,
        GameResult::Draw => 3,
    }
}

fn throw_score(throw: &Throw) -> i32 {
    match throw {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    }
}

fn decode_game_result(result: &char) -> GameResult {
    match result {
        'X' => Some(GameResult::Loss),
        'Y' => Some(GameResult::Draw),
        'Z' => Some(GameResult::Win),
        _ => None,
    }
    .unwrap()
}

fn throw_response(op: &Throw, result: &GameResult) -> Throw {
    match op {
        Throw::Rock => match result {
            GameResult::Win => Throw::Paper,
            GameResult::Loss => Throw::Scissors,
            GameResult::Draw => Throw::Rock,
        },
        Throw::Paper => match result {
            GameResult::Win => Throw::Scissors,
            GameResult::Loss => Throw::Rock,
            GameResult::Draw => Throw::Paper,
        },
        Throw::Scissors => match result {
            GameResult::Win => Throw::Rock,
            GameResult::Loss => Throw::Paper,
            GameResult::Draw => Throw::Scissors,
        },
    }
}

pub fn day2_p1(input: &str) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        let op = decode_op_throw(&chars[0]);
        let me = decode_my_throw(&chars[2]);
        let result = game_result(&me, &op);
        score += score_result(&result) + throw_score(&me);
    }

    score
}

pub fn day2_p2(input: &str) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        let result = decode_game_result(&chars[2]);
        let op = decode_op_throw(&chars[0]);
        let me = throw_response(&op, &result);
        score += score_result(&result) + throw_score(&me);
    }

    score
}
