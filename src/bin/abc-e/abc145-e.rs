use proconio::input;

fn main() {
    input! {
        (n, t): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by_key(|v| v.0);

    let mut ans = 0;
    let mut dp = vec![0; t];
    for &(a, b) in &ab {
        ans = ans.max(*dp.iter().max().unwrap() + b);
        for finish in (0..t.saturating_sub(a)).rev() {
            dp[finish + a] = dp[finish + a].max(dp[finish] + b);
        }
    }

    println!("{}", ans);
}
