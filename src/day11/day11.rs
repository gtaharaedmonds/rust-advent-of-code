#[derive(Debug)]
enum Op {
    Mult(i64),
    Add(i64),
    Square,
}

impl Op {
    fn exec(&self, old: i64) -> i64 {
        match self {
            Op::Mult(operator) => old * operator,
            Op::Add(operator) => old + operator,
            Op::Square => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    op: Op,
    test: i64,
    true_monkey: usize,
    false_monkey: usize,
}

const P1_ROUNDS: u64 = 20;
const P1_DIVISOR: i64 = 3;
const P2_ROUNDS: u64 = 10_000;
const P2_DIVISOR: i64 = 1;

pub fn day11_p1(input: &str) -> u64 {
    day11(input, P1_ROUNDS, P1_DIVISOR)
}

pub fn day11_p2(input: &str) -> u64 {
    day11(input, P2_ROUNDS, P2_DIVISOR)
}

fn day11(input: &str, rounds: u64, divisor: i64) -> u64 {
    let mut lines = input.lines().peekable();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items: Vec<Vec<i64>> = Vec::new();

    while lines.peek().is_some() {
        let items_line = lines.nth(1).unwrap().trim();
        let op_line = lines.next().unwrap().trim();
        let test_line = lines.next().unwrap().trim();
        let if_true = lines.next().unwrap().trim();
        let if_false = lines.next().unwrap().trim();
        lines.next();

        let starting_items: Vec<i64> = items_line
            .split(',')
            .collect::<String>()
            .split(' ')
            .skip(2)
            .map(|item| item.parse().unwrap())
            .collect();
        items.push(starting_items);

        let op = match op_line.split(' ').collect::<Vec<&str>>()[3..=5] {
            ["old", "*", "old"] => Some(Op::Square),
            ["old", "*", operator] => Some(Op::Mult(operator.parse().unwrap())),
            ["old", "+", operator] => Some(Op::Add(operator.parse().unwrap())),
            _ => None,
        }
        .unwrap();

        monkeys.push(Monkey {
            op,
            test: test_line.split(' ').nth(3).unwrap().parse().unwrap(),
            true_monkey: if_true.split(' ').nth(5).unwrap().parse().unwrap(),
            false_monkey: if_false.split(' ').nth(5).unwrap().parse().unwrap(),
        })
    }

    let mut inspections: Vec<u64> = vec![0; items.len()];
    let modulo: i64 = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..rounds {
        for (idx, monkey) in monkeys.iter().enumerate() {
            while !items[idx].is_empty() {
                let mut worry = items[idx].remove(0);
                worry = monkey.op.exec(worry);
                worry /= divisor;
                worry %= modulo;

                if worry % monkey.test == 0 {
                    items[monkey.true_monkey].push(worry);
                } else {
                    items[monkey.false_monkey].push(worry);
                }

                inspections[idx] += 1;
            }
        }
    }

    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}
