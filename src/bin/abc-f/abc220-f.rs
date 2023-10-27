use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut subtree_node_counts = vec![0; n];
    let mut sum_depth = 0;
    dfs(&graph, &mut subtree_node_counts, 0, &mut sum_depth, None, 0);

    let mut sum_dists = vec![0; n];
    let mut stack = vec![(0, sum_depth)];
    while let Some((cur, sum_dist)) = stack.pop() {
        if sum_dists[cur] != 0 {
            continue;
        }

        sum_dists[cur] = sum_dist;

        for &next in &graph[cur] {
            stack.push((next, sum_dist + n - 2 * subtree_node_counts[next]));
        }
    }

    println!("{}", sum_dists.iter().join("\n"));
}

fn dfs(
    graph: &[Vec<usize>],
    subtree_node_counts: &mut [usize],
    depth: usize,
    sum_depth: &mut usize,
    par: Option<usize>,
    cur: usize,
) {
    *sum_depth += depth;

    for &next in &graph[cur] {
        if par.is_some_and(|par| par == next) {
            continue;
        }

        dfs(
            graph,
            subtree_node_counts,
            depth + 1,
            sum_depth,
            Some(cur),
            next,
        );
        subtree_node_counts[cur] += subtree_node_counts[next];
    }

    subtree_node_counts[cur] += 1;
}
