use std::collections::VecDeque;

use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
        q: usize,
        xk: [(Usize1, usize); q],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = vec![0; q];
    let mut labels: Vec<Option<usize>> = vec![None; n];

    for (label, &(x, k)) in enumerate(&xk) {
        let mut sum = 0;
        let mut queue = VecDeque::from([(x, 0)]);

        while let Some((cur, dist)) = queue.pop_front() {
            if labels[cur] == Some(label) {
                continue;
            }

            labels[cur] = Some(label);

            sum += cur + 1;

            if dist < k {
                queue.extend(graph[cur].iter().map(|&next| (next, dist + 1)));
            }
        }

        ans[label] = sum;
    }

    println!("{}", ans.iter().join("\n"));
}
