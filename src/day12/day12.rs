use std::collections::VecDeque;

const HEIGHTS_MAP: &str = "abcdefghijklmnopqrstuvwxyz";

struct Node {
    pos: (i32, i32),
    step: u32,
}

pub fn day12_p1(input: &str) -> u32 {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        heights.push(Vec::new());

        for (x, height) in line.chars().enumerate() {
            heights[y].push(match height {
                'S' => {
                    start = (x as i32, y as i32);
                    0
                }
                'E' => {
                    end = (x as i32, y as i32);
                    25
                }
                _ => HEIGHTS_MAP.chars().position(|c| c == height).unwrap() as i32,
            })
        }
    }

    shortest_path(start, end, &heights).unwrap()
}

pub fn day12_p2(input: &str) -> u32 {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    let mut starts: Vec<(i32, i32)> = Vec::new();
    let mut end: (i32, i32) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        heights.push(Vec::new());

        for (x, height) in line.chars().enumerate() {
            heights[y].push(match height {
                'S' | 'a' => {
                    starts.push((x as i32, y as i32));
                    0
                }
                'E' => {
                    end = (x as i32, y as i32);
                    25
                }
                _ => HEIGHTS_MAP.chars().position(|c| c == height).unwrap() as i32,
            })
        }
    }

    starts
        .iter()
        .filter_map(|start| shortest_path(*start, end, &heights))
        .min()
        .unwrap()
}

fn shortest_path(start: (i32, i32), end: (i32, i32), heights: &Vec<Vec<i32>>) -> Option<u32> {
    let m = heights.len() as i32;
    let n = heights[0].len() as i32;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n as usize]; m as usize];
    visited[start.1 as usize][start.0 as usize] = true;

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(Node {
        pos: (start.0 as i32, start.1 as i32),
        step: 0,
    });

    while let Some(node) = queue.pop_front() {
        if node.pos == end {
            return Some(node.step);
        }

        for dir in dirs {
            let pos = (node.pos.0 + dir.0, node.pos.1 + dir.1);

            if pos.0 < 0 || pos.1 < 0 || pos.0 >= n || pos.1 >= m {
                // Invalid position
                continue;
            }

            if visited[pos.1 as usize][pos.0 as usize] {
                // Already seen node
                continue;
            }

            if heights[pos.1 as usize][pos.0 as usize]
                - heights[node.pos.1 as usize][node.pos.0 as usize]
                > 1
            {
                // Too high up
                continue;
            }

            queue.push_back(Node {
                pos,
                step: node.step + 1,
            });
            visited[pos.1 as usize][pos.0 as usize] = true;
        }
    }

    None
}
