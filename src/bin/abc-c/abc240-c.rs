use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        ab: [(usize, usize); n],
    }

    let mut dp = vec![false; x + 1];
    dp[0] = true;

    for &(a, b) in &ab {
        let mut next_dp = vec![false; x + 1];
        for i in a..=x {
            next_dp[i] |= dp[i - a];
        }
        for i in b..=x {
            next_dp[i] |= dp[i - b];
        }

        dp = next_dp;
    }

    println!("{}", if dp[x] { "Yes" } else { "No" });
}
