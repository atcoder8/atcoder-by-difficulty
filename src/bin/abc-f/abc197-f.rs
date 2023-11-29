use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, char); m],
    }

    let char_to_usize = |c: char| (c as u8 - b'a') as usize;

    let mut graph = vec![vec![vec![]; 26]; n];
    for &(a, b, c) in &abc {
        graph[a][char_to_usize(c)].push(b);
        graph[b][char_to_usize(c)].push(a);
    }

    let mut ans = None;
    let mut dists = vec![vec![None; n]; n];
    let mut queue = VecDeque::from([(0, n - 1, 0)]);
    while let Some((node1, node2, dist)) = queue.pop_front() {
        if dists[node1][node2].is_some() {
            continue;
        }

        dists[node1][node2] = Some(dist);

        if node1 == node2 {
            update_cost(&mut ans, 2 * dist);
        }

        for c in 0..26 {
            for (&next_node_1, &next_node_2) in iproduct!(&graph[node1][c], &graph[node2][c]) {
                queue.push_back((next_node_1, next_node_2, dist + 1));
            }
        }
    }

    for &(a, b, _) in &abc {
        if let Some(dist) = dists[a][b] {
            update_cost(&mut ans, 2 * dist + 1);
        }

        if let Some(dist) = dists[b][a] {
            update_cost(&mut ans, 2 * dist + 1);
        }
    }

    ans
}

pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: Ord,
{
    if cost.as_ref().is_some_and(|cost| &cand_cost > cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
