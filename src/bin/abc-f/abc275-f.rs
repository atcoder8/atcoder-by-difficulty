use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    // `dp[i][j]` は次の条件を満たすように削除しないAの要素を選択する方法のうち選択されなかった部分列の個数の最小値を表す
    // * 選択された要素の和がi
    // * 現在参照している要素を選択する <=> j == 1
    // ただし、そのような選択の方法が存在しない場合は `None`
    let mut dp = vec![vec![None; 2]; m + 1];
    dp[0][1] = Some(0);
    for &a in &aa {
        let mut next_dp = vec![vec![None; 2]; m + 1];
        for (sum, from, to) in iproduct!(0..=m, 0..2, 0..2) {
            if let Some(cost) = dp[sum][from] {
                let next_sum = sum + a * to;

                if next_sum > m {
                    continue;
                }

                let next_cost = &mut next_dp[next_sum][to];
                let cand_cost = cost + (from == 1 && to == 0) as usize;
                if next_cost.is_none() || cand_cost < next_cost.unwrap() {
                    *next_cost = Some(cand_cost);
                }
            }
        }

        dp = next_dp;
    }

    for x in 1..=m {
        match dp[x].iter().filter_map(|&cost| cost).min() {
            Some(ans) => println!("{}", ans),
            None => println!("-1"),
        }
    }
}
