use std::cmp::{max, min};

pub fn day10_p1(input: &str) -> i32 {
    let mut cycle = 1;
    let mut sum = 0;
    let mut x = 1;

    let cycle_value = |x: i32, cycle: i32| -> i32 {
        if ![20, 60, 100, 140, 180, 220].contains(&cycle) {
            return 0;
        }

        x * cycle
    };

    for instruction in input.lines() {
        if instruction == "noop" {
            cycle += 1;
            sum += cycle_value(x, cycle);
        } else {
            cycle += 1;
            sum += cycle_value(x, cycle);

            cycle += 1;
            x += instruction
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            sum += cycle_value(x, cycle);
        }
    }

    sum
}

pub fn day10_p2(input: &str) -> String {
    let mut screen: Vec<Vec<char>> = vec![vec!['_'; 40]; 6];
    let mut pixel: i32 = 0;
    let mut x = 0;

    let sprite_on_pixel = |pixel: &i32, x: &i32| -> bool {
        let pixel = (pixel % 40) as i32;
        let x = (x % 40) as i32;
        pixel >= max(x - 1, 0) && pixel <= min(x + 1, 39)
    };

    for instruction in input.lines() {
        if instruction == "noop" {
            screen[pixel as usize / 40][pixel as usize % 40] = match sprite_on_pixel(&pixel, &x) {
                true => '#',
                false => '.',
            };
            pixel += 1;
        } else {
            screen[pixel as usize / 40][pixel as usize % 40] = match sprite_on_pixel(&pixel, &x) {
                true => '#',
                false => '.',
            };
            pixel += 1;

            x += instruction
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            screen[pixel as usize / 40][pixel as usize % 40] = match sprite_on_pixel(&pixel, &x) {
                true => '#',
                false => '.',
            };
            pixel += 1;
        }
    }

    let mut img = String::new();
    for row in screen {
        img += &row.iter().collect::<String>();
        img += "\n";
    }

    img
}
