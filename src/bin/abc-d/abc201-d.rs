use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    println!(
        "{}",
        match solve() {
            Ordering::Greater => "Takahashi",
            Ordering::Less => "Aoki",
            Ordering::Equal => "Draw",
        }
    );
}

fn solve() -> Ordering {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
    }

    let effect = |i: usize, j: usize| {
        if i == 0 && j == 0 {
            0
        } else if (aaa[i][j] == '+') ^ ((i + j) % 2 == 0) {
            1
        } else {
            -1
        }
    };

    let mut dp = vec![vec![0; w]; h];
    dp[h - 1][w - 1] = effect(h - 1, w - 1);
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 && j == w - 1 {
                continue;
            }

            let mut candidates = vec![];
            if i < h - 1 {
                candidates.push(dp[i + 1][j]);
            }
            if j < w - 1 {
                candidates.push(dp[i][j + 1]);
            }

            dp[i][j] = if (i + j) % 2 == 0 {
                candidates.iter().max().unwrap()
            } else {
                candidates.iter().min().unwrap()
            } + effect(i, j);
        }
    }

    dp[0][0].cmp(&0)
}
