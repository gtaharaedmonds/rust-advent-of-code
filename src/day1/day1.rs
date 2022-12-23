use std::cmp::max;

pub fn day1_p1(input: &str) -> u64 {
    let mut max_cals: u64 = 0;
    let mut cals: u64 = 0;

    let input: String = input.to_string() + "\n\n"; // So calories are checked on last line

    for line in input.lines() {
        if line.is_empty() {
            max_cals = max(cals, max_cals);
            cals = 0;
        } else {
            cals += line.parse::<u64>().unwrap();
        }
    }

    max_cals
}

pub fn day1_p2(input: &str) -> u64 {
    let mut max_vec: Vec<u64> = vec![0, 0, 0];
    let mut cals = 0;

    let input: String = input.to_string() + "\n\n"; // So calories are checked on last line

    for line in input.lines() {
        if line.is_empty() {
            for i in 0..3 {
                if cals > max_vec[i] {
                    max_vec.insert(i, cals);
                    max_vec.pop();
                    break;
                }
            }

            cals = 0;
        } else {
            cals += line.parse::<u64>().unwrap();
        }
    }

    max_vec.iter().sum()
}
