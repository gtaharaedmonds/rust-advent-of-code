use std::cmp::max;

pub fn day8_p1(input: &str) -> usize {
    let size = input.lines().next().unwrap().len();
    let mut heights: Vec<Vec<i32>> = vec![vec![0; size]; size];
    let mut visible: Vec<Vec<bool>> = vec![vec![false; size]; size];

    // Parse table
    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            heights[y][x] = height.to_digit(10).unwrap() as i32;
        }
    }

    // Function for finding all visible given starting position, walking direction, and looking direction
    fn mark_visible(
        start: (i32, i32),
        move_dir: (i32, i32),
        look_dir: (i32, i32),
        heights: &Vec<Vec<i32>>,
        visible: &mut [Vec<bool>],
    ) {
        let size = heights.len();
        let mut pos = (start.0, start.1);

        for _ in 0..size {
            let mut highest: i32 = -1;
            let mut target = pos;

            for _ in 0..size {
                let height = heights[target.1 as usize][target.0 as usize] as i32;
                if height > highest {
                    visible[target.1 as usize][target.0 as usize] = true;
                    highest = max(highest, height);
                }

                target.0 += look_dir.0;
                target.1 += look_dir.1;
            }

            pos.0 += move_dir.0;
            pos.1 += move_dir.1;
        }
    }

    // Find all visible
    let size = size as i32;
    mark_visible((0, 0), (0, 1), (1, 0), &heights, &mut visible[..]);
    mark_visible((0, 0), (1, 0), (0, 1), &heights, &mut visible[..]);
    mark_visible((0, size - 1), (1, 0), (0, -1), &heights, &mut visible[..]);
    mark_visible((size - 1, 0), (0, 1), (-1, 0), &heights, &mut visible[..]);

    // Count all visible
    visible.iter().flatten().filter(|pos| **pos).count()
}

pub fn day8_p2(input: &str) -> u32 {
    let size = input.lines().next().unwrap().len();
    let mut heights: Vec<Vec<i32>> = vec![vec![0; size]; size];

    // Parse table
    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            heights[y][x] = height.to_digit(10).unwrap() as i32;
        }
    }

    // Count number of trees visible along a direction from a given position
    fn count_visible(start: (i32, i32), dir: (i32, i32), heights: &Vec<Vec<i32>>) -> u32 {
        let size = heights.len() as i32;
        let height = heights[start.1 as usize][start.0 as usize];
        let mut target = (start.0 + dir.0, start.1 + dir.1);
        let mut count = 0;

        while target.0 >= 0 && target.0 < size && target.1 >= 0 && target.1 < size {
            count += 1;

            if heights[target.1 as usize][target.0 as usize] >= height {
                // Break out if view is broken, but include the tree that breaks view in the count
                break;
            }

            target.0 += dir.0;
            target.1 += dir.1;
        }

        count
    }

    // Calculate max score
    let mut score = 0;
    for y in 0..size {
        for x in 0..size {
            let start = (x as i32, y as i32);

            let up = count_visible(start, (0, -1), &heights);
            let down = count_visible(start, (0, 1), &heights);
            let right = count_visible(start, (1, 0), &heights);
            let left = count_visible(start, (-1, 0), &heights);

            score = max(score, up * down * right * left);
        }
    }

    score
}
