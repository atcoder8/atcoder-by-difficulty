use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, t): (usize, usize, usize),
        aa: [usize; n],
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    let mut rev_graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        rev_graph[b].push((a, c));
    }

    let mut dists = vec![None; n];
    dists[0] = Some(0);
    let mut heap = BinaryHeap::from(vec![(Reverse(0), 0)]);
    while let Some((Reverse(dist), cur)) = heap.pop() {
        if dists[cur] != Some(dist) {
            continue;
        }

        for &(next, cost) in &graph[cur] {
            if dists[next].is_none() || dist + cost < dists[next].unwrap() {
                dists[next] = Some(dist + cost);
                heap.push((Reverse(dist + cost), next));
            }
        }
    }

    let mut rev_dists = vec![None; n];
    rev_dists[0] = Some(0);
    let mut heap = BinaryHeap::from(vec![(Reverse(0), 0)]);
    while let Some((Reverse(dist), cur)) = heap.pop() {
        if rev_dists[cur] != Some(dist) {
            continue;
        }

        for &(next, cost) in &rev_graph[cur] {
            if rev_dists[next].is_none() || dist + cost < rev_dists[next].unwrap() {
                rev_dists[next] = Some(dist + cost);
                heap.push((Reverse(dist + cost), next));
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if let (Some(dist1), Some(dist2)) = (dists[i], rev_dists[i]) {
            ans = ans.max(aa[i] * t.saturating_sub(dist1 + dist2));
        }
    }
    println!("{}", ans);
}
