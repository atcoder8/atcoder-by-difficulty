use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; m],
    }

    let mut dp = vec![vec![None; m + 1]; n + 1];
    dp[0][0] = Some(0);
    for i in 1..=n {
        dp[i][0] = Some(i);
    }
    for j in 1..=m {
        dp[0][j] = Some(j);
    }

    for (i, j) in iproduct!(1..=n, 1..=m) {
        if let Some(prev_cost) = dp[i - 1][j] {
            update_cost(&mut dp[i][j], prev_cost + 1);
        }

        if let Some(prev_cost) = dp[i][j - 1] {
            update_cost(&mut dp[i][j], prev_cost + 1);
        }

        if let Some(prev_cost) = dp[i - 1][j - 1] {
            update_cost(&mut dp[i][j], prev_cost + (aa[i - 1] != bb[j - 1]) as usize);
        }
    }

    println!("{}", dp[n][m].unwrap());
}

/// Updates the minimum cost.
/// If `cost` is `None`, always updated to the candidate cost.
///
/// # Arguments
///
/// * `cost` - Reference variable for the cost to be updated.
/// * `cand_cost` - Candidate cost to update.
pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: PartialOrd,
{
    if cost.as_ref().is_some_and(|cost| cost <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
