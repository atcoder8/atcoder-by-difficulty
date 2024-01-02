use std::collections::VecDeque;

use itertools::{iproduct, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
        k: usize,
        cc: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let calc_dists = |c: usize| {
        let mut dists = vec![None; n];
        let mut queue = VecDeque::from([(c, 0)]);
        while let Some((cur, dist)) = queue.pop_front() {
            if update_cost(&mut dists[cur], dist) {
                queue.extend(graph[cur].iter().map(|&next| (next, dist + 1)));
            }
        }

        dists
    };

    let dists_per_c = cc.iter().map(|&c| calc_dists(c)).collect_vec();

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; k]; 1 << k];
    for i in 0..k {
        dp[1 << i][i] = Some(1);
    }
    for (bit, from, to) in iproduct!(0..1 << k, 0..k, 0..k) {
        if let (Some(sum_dist), Some(dist)) = (dp[bit][from], dists_per_c[from][cc[to]]) {
            update_cost(&mut dp[bit | 1 << to][to], sum_dist + dist);
        }
    }

    let ans = dp[(1 << k) - 1]
        .iter()
        .filter_map(|&sum_dist| sum_dist)
        .min();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
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
