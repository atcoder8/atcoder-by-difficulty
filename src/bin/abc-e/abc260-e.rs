use std::collections::VecDeque;

use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); n],
    }

    let mut graph = vec![vec![]; m];
    for (i, &(a, b)) in ab.iter().enumerate() {
        graph[a].push(i);
        graph[b].push(i);
    }

    let mut imos = vec![0_i64; m + 1];
    let mut counts = vec![0; n];
    let mut cnt = 0;
    let mut queue = VecDeque::new();
    for r in 0..m {
        queue.push_back(r);

        for &i in &graph[r] {
            if counts[i] == 0 {
                cnt += 1;
            }
            counts[i] += 1;
        }

        while cnt == n {
            let l = queue.pop_front().unwrap();
            for &i in &graph[l] {
                counts[i] -= 1;
                if counts[i] == 0 {
                    cnt -= 1;
                }
            }
        }

        imos[queue.len()] += 1;
        imos[r + 1] -= 1;
    }

    for i in 0..(m - 1) {
        imos[i + 1] += imos[i];
    }

    println!("{}", join(imos[..m].iter(), " "));
}
