pub fn day6_p1(input: &str) -> usize {
    let input: Vec<char> = input.chars().collect();
    let mut buf: Vec<char> = Vec::new();

    for (i, c) in input.iter().enumerate() {
        if buf.contains(c) {
            while buf[0] != *c {
                buf.remove(0);
            }

            buf.remove(0);
        }

        buf.push(*c);

        if buf.len() >= 4 {
            return i + 1;
        }
    }

    0
}

pub fn day6_p2(input: &str) -> usize {
    let input: Vec<char> = input.chars().collect();
    let mut buf: Vec<char> = Vec::new();

    for (i, c) in input.iter().enumerate() {
        if buf.contains(c) {
            while buf[0] != *c {
                buf.remove(0);
            }

            buf.remove(0);
        }

        buf.push(*c);

        if buf.len() >= 14 {
            return i + 1;
        }
    }

    0
}
