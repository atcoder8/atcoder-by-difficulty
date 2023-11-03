use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut min_costs: Vec<Vec<Option<usize>>> = vec![vec![None; n]; 1 << n];
    min_costs[0].iter_mut().for_each(|cost| *cost = Some(0));

    let mut queue: VecDeque<_> = (0..n).map(|i| (1 << i, i, 1)).collect();
    while let Some((s, tail, cost)) = queue.pop_front() {
        let next_cost = &mut min_costs[s][tail];

        if next_cost.is_some_and(|next_cost| next_cost <= cost) {
            continue;
        }

        *next_cost = Some(cost);

        for &next in &graph[tail] {
            queue.push_back((s ^ 1 << next, next, cost + 1));
        }
    }

    let ans: usize = min_costs
        .iter()
        .map(|min_cost| {
            min_cost
                .iter()
                .filter_map(|&min_cost| min_cost)
                .min()
                .unwrap()
        })
        .sum();
    println!("{}", ans);
}
