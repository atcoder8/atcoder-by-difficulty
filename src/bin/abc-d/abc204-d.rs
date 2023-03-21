use proconio::input;

fn main() {
    input! {
        n: usize,
        tt: [usize; n],
    }

    let sum_t: usize = tt.iter().sum();

    let mut dp = vec![false; sum_t + 1];
    dp[0] = true;
    for &t in &tt {
        let mut next_dp = dp.clone();
        for i in t..=sum_t {
            next_dp[i] |= dp[i - t];
        }
        dp = next_dp;
    }

    let mut ans = sum_t;
    for i in 0..=sum_t {
        if dp[i] {
            ans = ans.min(i.max(sum_t - i));
        }
    }

    println!("{}", ans);
}
