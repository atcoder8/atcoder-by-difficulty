use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (h, w): (usize, usize),
        grid: [Chars; h],
    }

    let mut teleporters = vec![vec![]; 26];
    for (row, col) in iproduct!(0..h, 0..w) {
        let c = grid[row][col];
        if c.is_lowercase() {
            teleporters[char_to_usize(c)].push((row, col));
        }
    }

    let start_coord = iproduct!(0..h, 0..w)
        .find(|&(row, col)| grid[row][col] == 'S')
        .unwrap();
    let goal_coord = iproduct!(0..h, 0..w)
        .find(|&(row, col)| grid[row][col] == 'G')
        .unwrap();

    let mut visited = vec![vec![false; w]; h];
    let mut used = vec![false; 26];
    let mut queue = VecDeque::from([(start_coord, 0)]);
    while let Some(((row, col), cand_dist)) = queue.pop_front() {
        if visited[row][col] {
            continue;
        }

        visited[row][col] = true;

        if (row, col) == goal_coord {
            return Some(cand_dist);
        }

        let c = grid[row][col];

        if c.is_lowercase() {
            let teleporter = char_to_usize(c);

            if !used[teleporter] {
                used[teleporter] = true;

                for &next_coord in &teleporters[teleporter] {
                    queue.push_back((next_coord, cand_dist + 1));
                }
            }
        }

        for (diff_row, diff_col) in DIFFS {
            let next_row = row.wrapping_add(diff_row);
            let next_col = col.wrapping_add(diff_col);

            if next_row < h && next_col < w && grid[next_row][next_col] != '#' {
                queue.push_back(((next_row, next_col), cand_dist + 1));
            }
        }
    }

    None
}

fn char_to_usize(c: char) -> usize {
    (c as u8 - b'a') as usize
}
