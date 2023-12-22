use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let dists_from_node0 = calc_dist(&graph, 0);
    let end1 = (0..n).max_by_key(|&node| dists_from_node0[node]).unwrap();
    let dists_from_end1 = calc_dist(&graph, end1);
    let end2 = (0..n).max_by_key(|&node| dists_from_end1[node]).unwrap();
    let dists_from_end2 = calc_dist(&graph, end2);

    graph.iter_mut().for_each(|children| {
        children.sort_unstable_by_key(|&next| dists_from_end2[next]);
    });

    enum DFSNode {
        Forward { par: Option<usize>, cur: usize },
        Backward,
    }

    let mut labels = vec![0; n];
    let mut stack = vec![DFSNode::Forward {
        par: None,
        cur: end1,
    }];
    let mut label = 0;
    while let Some(dfs_node) = stack.pop() {
        label += 1;

        let DFSNode::Forward { par, cur } = dfs_node else { continue; };

        labels[cur] = label;

        for &next in &graph[cur] {
            if Some(next) != par {
                stack.push(DFSNode::Backward);
                stack.push(DFSNode::Forward {
                    par: Some(cur),
                    cur: next,
                });
            }
        }
    }

    println!("{}", labels.iter().join(" "));
}

fn calc_dist(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = graph.len();

    let mut dists = vec![n; n];
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((cur, cand_dist)) = queue.pop_front() {
        if dists[cur] <= cand_dist {
            continue;
        }

        dists[cur] = cand_dist;

        for &next in &graph[cur] {
            queue.push_back((next, cand_dist + 1));
        }
    }

    dists
}
