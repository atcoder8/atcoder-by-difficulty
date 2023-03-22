use proconio::input;

fn main() {
    input! {
        (h, w, c): (usize, usize, usize),
        aaa: [[usize; w]; h],
    }

    let mut ans = 2e18 as usize;

    let mut dp = vec![vec![2e18 as usize; w]; h];

    for i in 0..h {
        for j in 0..w {
            if i > 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + c);
            }

            if j > 0 {
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + c);
            }

            ans = ans.min(dp[i][j] + aaa[i][j]);

            dp[i][j] = dp[i][j].min(aaa[i][j]);
        }
    }

    let mut dp = vec![vec![2e18 as usize; w]; h];

    for i in 0..h {
        for j in (0..w).rev() {
            if i > 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + c);
            }

            if j < w - 1 {
                dp[i][j] = dp[i][j].min(dp[i][j + 1] + c);
            }

            ans = ans.min(dp[i][j] + aaa[i][j]);

            dp[i][j] = dp[i][j].min(aaa[i][j]);
        }
    }

    println!("{}", ans);
}
