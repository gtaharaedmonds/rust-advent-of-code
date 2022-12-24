use regex::Regex;
use std::collections::HashSet;

pub fn day5_p1(input: &str) -> String {
    // Set for checking if a given character is a crate (is a letter)
    let crate_set: HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    // Regex for crate move instructions
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // Define data structure (vec of vecs) to hold crates
    let width = input.lines().next().unwrap().len();
    let mut crates: Vec<Vec<char>> = vec![Vec::new(); width];

    // Iterator over lines
    let mut lines = input.lines();

    // Parse initial crate layout
    loop {
        let line: Vec<char> = lines.next().unwrap().chars().collect();
        if !line.contains(&'[') {
            // We've passed the initial crate layout section, break
            break;
        }

        for (i, crate_char) in line.iter().skip(1).step_by(4).enumerate() {
            // Iterate over all crate letter positions, add new crate if is indeed a letter
            if crate_set.contains(crate_char) {
                crates[i].insert(0, *crate_char);
            }
        }
    }

    lines.next();

    // Execute crate moving instructions
    for line in lines {
        let captures = re.captures(line).unwrap();
        let count: usize = captures[1].parse().unwrap();
        let src: usize = captures[2].parse::<usize>().unwrap() - 1;
        let dest: usize = captures[3].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            // Move count crates from stack src to stack dest
            let moved = crates[src].pop().unwrap();
            crates[dest].push(moved);
        }
    }

    // Get top of each stack, as a single string
    crates.iter().filter_map(|stack| stack.last()).collect()
}

pub fn day5_p2(input: &str) -> String {
    // Set for checking if a given character is a crate (is a letter)
    let crate_set: HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    // Regex for crate move instructions
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // Define data structure (vec of vecs) to hold crates
    let width = input.lines().next().unwrap().len();
    let mut crates: Vec<Vec<char>> = vec![Vec::new(); width];

    // Iterator over lines
    let mut lines = input.lines();

    // Parse initial crate layout
    loop {
        let line: Vec<char> = lines.next().unwrap().chars().collect();
        if !line.contains(&'[') {
            // We've passed the initial crate layout section, break
            break;
        }

        for (i, crate_char) in line.iter().skip(1).step_by(4).enumerate() {
            // Iterate over all crate letter positions, add new crate if is indeed a letter
            if crate_set.contains(crate_char) {
                crates[i].insert(0, *crate_char);
            }
        }
    }

    lines.next();

    // Execute crate moving instructions
    for line in lines {
        let captures = re.captures(line).unwrap();
        let count: usize = captures[1].parse().unwrap();
        let src: usize = captures[2].parse::<usize>().unwrap() - 1;
        let dest: usize = captures[3].parse::<usize>().unwrap() - 1;

        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..count {
            // Move count crates from stack src to temp stack
            tmp.push(crates[src].pop().unwrap());
        }

        for _ in 0..count {
            // Move count crates temp stack to dest stack
            // Reverse order 2x, so don't actually reverse order
            crates[dest].push(tmp.pop().unwrap());
        }

        // Could probably do in 1 line with iterators, but this is clearer
    }

    // Get top of each stack, as a single string
    crates.iter().filter_map(|stack| stack.last()).collect()
}
