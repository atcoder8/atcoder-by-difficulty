use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let mut costs = vec![vec![h + w; w]; h];
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::from([(0, 0, 0)]);
    while let Some((row, col, cost)) = queue.pop_front() {
        if cost >= costs[row][col] {
            continue;
        }

        costs[row][col] = cost;

        for (diff_row, diff_col) in DIFFS {
            let (next_row, next_col) = (row.wrapping_add(diff_row), col.wrapping_add(diff_col));

            if next_row < h && next_col < w && sss[next_row][next_col] == '.' {
                queue.push_front((next_row, next_col, cost));
            }
        }

        for (i, j) in (0..5).cartesian_product(0..5) {
            if (i == 0 || i == 4) && (j == 0 || j == 4) {
                continue;
            }

            let (next_row, next_col) = ((row + i).wrapping_sub(2), (col + j).wrapping_sub(2));
            if next_row < h && next_col < w {
                queue.push_back((next_row, next_col, cost + 1));
            }
        }
    }

    println!("{}", costs[h - 1][w - 1]);
}
