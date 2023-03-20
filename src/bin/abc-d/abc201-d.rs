use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
    }

    let mut effect = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            effect[i][j] = if (aaa[i][j] == '+') == ((i + j) % 2 == 1) {
                1
            } else {
                -1
            };
        }
    }
    effect[0][0] = 0;

    let mut dp = vec![vec![0; w]; h];
    dp[h - 1][w - 1] = effect[h - 1][w - 1];

    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if (i, j) == (h - 1, w - 1) {
                continue;
            }

            let mut candidate = vec![];
            if i < h - 1 {
                candidate.push(dp[i + 1][j]);
            }
            if j < w - 1 {
                candidate.push(dp[i][j + 1]);
            }

            let mut best = if (i + j) % 2 == 0 {
                candidate.into_iter().max().unwrap()
            } else {
                candidate.into_iter().min().unwrap()
            };

            best += effect[i][j];

            dp[i][j] = best;
        }
    }

    if dp[0][0] > 0 {
        println!("Takahashi");
    } else if dp[0][0] < 0 {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
