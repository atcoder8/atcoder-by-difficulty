use std::collections::VecDeque;

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

    let mut dp = vec![(n, 0_usize); n];
    dp[0] = (0, 1);
    let mut queue = VecDeque::from(vec![0_usize]);

    while let Some(node) = queue.pop_front() {
        let (dist, cnt) = dp[node];

        for &next_node in &graph[node] {
            let (next_dist, next_cnt) = &mut dp[next_node];

            if dist + 1 < *next_dist {
                *next_dist = dist + 1;
                *next_cnt = cnt;

                queue.push_back(next_node);
            } else if dist + 1 == *next_dist {
                *next_cnt = (*next_cnt + cnt) % 1000000007;
            }
        }
    }

    println!("{}", dp[n - 1].1);
}
