use std::collections::VecDeque;

use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut parents: Vec<Option<usize>> = vec![None; n];

    let mut queue = VecDeque::from(vec![0_usize]);

    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if parents[next].is_some() {
                continue;
            }

            parents[next] = Some(cur);
            queue.push_back(next);
        }
    }

    let ans = parents[1..].iter().map(|&parent| parent.unwrap() + 1);
    println!("Yes\n{}", join(ans, "\n"));
}
