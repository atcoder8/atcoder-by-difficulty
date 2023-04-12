use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    let mut indegree = vec![0; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        indegree[b] += 1;
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut ans = vec![];
    let mut heap: BinaryHeap<_> = (0..n)
        .filter(|&i| indegree[i] == 0)
        .map(|i| Reverse(i))
        .collect();
    while let Some(Reverse(cur)) = heap.pop() {
        ans.push(cur);

        for &next_node in &graph[cur] {
            indegree[next_node] -= 1;

            if indegree[next_node] == 0 {
                heap.push(Reverse(next_node));
            }
        }
    }

    if ans.len() == n {
        println!("{}", ans.iter().map(|x| (x + 1).to_string()).join(" "));
    } else {
        println!("-1");
    }
}
