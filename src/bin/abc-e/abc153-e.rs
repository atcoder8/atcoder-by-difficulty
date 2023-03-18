use proconio::input;

fn main() {
    input! {
        (h, n): (usize, usize),
        ab: [(usize, usize); n],
    }

    let mut dp = vec![10_usize.pow(8); h + 1];
    dp[0] = 0;
    for &(a, b) in &ab {
        for i in 0..h {
            let next = (i + a).min(h);
            dp[next] = dp[next].min(dp[i] + b);
        }
    }

    let ans = dp[h];
    println!("{}", ans);
}
