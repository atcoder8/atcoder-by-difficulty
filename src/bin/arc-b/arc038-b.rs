use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let mut dp = vec![vec![false; w]; h];
    for (i, j) in iproduct!((0..h).rev(), (0..w).rev()) {
        if sss[i][j] == '.' {
            dp[i][j] = i < h - 1 && sss[i + 1][j] == '.' && !dp[i + 1][j]
                || j < w - 1 && sss[i][j + 1] == '.' && !dp[i][j + 1]
                || i < h - 1 && j < w - 1 && sss[i + 1][j + 1] == '.' && !dp[i + 1][j + 1];
        }
    }

    println!("{}", if dp[0][0] { "First" } else { "Second" });
}
