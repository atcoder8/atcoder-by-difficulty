use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        ab: [(Usize1, Usize1); n - 1],
        px: [(Usize1, usize); q],
    }

    let mut counts = vec![0; n];
    for &(p, x) in &px {
        counts[p] += x;
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut stack = vec![(None, 0, 0)];
    while let Some((par, cur, sum)) = stack.pop() {
        counts[cur] += sum;

        for &next in &graph[cur] {
            if Some(next) != par {
                stack.push((Some(cur), next, counts[cur]));
            }
        }
    }

    println!("{}", counts.iter().join(" "));
}
