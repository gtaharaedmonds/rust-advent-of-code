use std::collections::HashSet;

pub fn day3_p1(input: &str) -> i32 {
    let item_list = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let mut score: i32 = 0;

    for line in input.lines() {
        let items = line.as_bytes();

        // Create set of first N/2 elements
        let comp1_set: HashSet<u8> = items.iter().take(line.len()/2).copied().collect();

        // Continue iterating over next N/2 elements until match is found
        let bad_item = items.iter().skip(line.len()/2).find(|x| comp1_set.contains(x)).unwrap();

        score += item_list.iter().position(|x| x == bad_item).unwrap() as i32 + 1;
    }

    score
}

pub fn day3_p2(input: &str) -> i32 {
    let item_list = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let mut score: i32 = 0;

    for i in 0..lines.len()/3 {
        let sack1 = lines[i*3];
        let sack2 = lines[i*3+1];
        let sack3 = lines[i*3+2];

        let sack1_set: HashSet<u8> = sack1.iter().copied().collect(); // Make a hashset for sack1, for O(1) lookup
        let sack2_set: HashSet<u8> = sack2.iter().copied().collect(); // Make a hashset for sack2, for O(1) lookup
        
        // Bad item is whichever is in all sacks
        let bad_item = sack3.iter().find(|x| sack1_set.contains(x) && sack2_set.contains(x)).unwrap();

        score += item_list.iter().position(|x| x == bad_item).unwrap() as i32 + 1;
    }

    score
}