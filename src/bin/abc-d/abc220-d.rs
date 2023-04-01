use itertools::join;
use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut dp = vec![0; 10];
    dp[aa[0]] = 1;

    for &a in aa[1..].iter() {
        let mut next_dp = vec![0; 10];

        for i in 0..10 {
            next_dp[(i + a) % 10] = (next_dp[(i + a) % 10] + dp[i]) % MOD;
            next_dp[i * a % 10] = (next_dp[i * a % 10] + dp[i]) % MOD;
        }

        dp = next_dp;
    }

    println!("{}", join(dp, "\n"));
}
