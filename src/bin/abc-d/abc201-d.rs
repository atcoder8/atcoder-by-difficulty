use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
    }

    let mut dp = vec![vec![0; w]; h];
    dp[h - 1][w - 1] = if (aaa[h - 1][w - 1] == '+') ^ ((h + w) % 2 == 1) {
        -1
    } else {
        1
    };

    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 && j == w - 1 {
                continue;
            }

            if (i + j) % 2 == 0 {
                let mut best = -((h * w) as i32);

                if i < h - 1 {
                    best = best.max(dp[i + 1][j]);
                }

                if j < w - 1 {
                    best = best.max(dp[i][j + 1]);
                }

                best += if aaa[i][j] == '+' { -1 } else { 1 };

                dp[i][j] = best;
            } else {
                let mut best = (h * w) as i32;

                if i < h - 1 {
                    best = best.min(dp[i + 1][j]);
                }

                if j < w - 1 {
                    best = best.min(dp[i][j + 1]);
                }

                best += if aaa[i][j] == '+' { 1 } else { -1 };

                dp[i][j] = best;
            }
        }
    }

    dp[0][0] -= if aaa[0][0] == '+' { -1 } else { 1 };

    if dp[0][0] > 0 {
        println!("Takahashi");
    } else if dp[0][0] < 0 {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
