use std::{cmp::Reverse, collections::BinaryHeap};

use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abcd: [(Usize1, Usize1, usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, c, d) in &abcd {
        graph[a].push((b, c, d));
        graph[b].push((a, c, d));
    }

    let mut costs = vec![None; n];
    let mut heap = BinaryHeap::from(vec![(Reverse(0), 0)]);
    while let Some((Reverse(cur_cost), cur)) = heap.pop() {
        if costs[cur].is_some() && cur_cost >= costs[cur].unwrap() {
            continue;
        }

        costs[cur] = Some(cur_cost);

        for &(next, c, d) in &graph[cur] {
            let waited_times = [
                cur_cost.max(d.sqrt()),
                cur_cost.max(d.sqrt().saturating_sub(1)),
            ];
            let candidate_cost = waited_times
                .iter()
                .map(|&waited| waited + c + d / (waited + 1))
                .min()
                .unwrap();
            if costs[next].is_none() || candidate_cost < costs[next].unwrap() {
                heap.push((Reverse(candidate_cost), next));
            }
        }
    }

    if let Some(ans) = costs[n - 1] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
