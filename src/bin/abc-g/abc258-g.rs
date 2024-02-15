use fixedbitset::FixedBitSet;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        adj_mat: [Chars; n],
    }

    let graph = adj_mat
        .iter()
        .map(|line| {
            let mut edge = FixedBitSet::with_capacity(n);
            (0..n)
                .filter(|&i| line[i] == '1')
                .for_each(|i| edge.insert(i));

            edge
        })
        .collect_vec();

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if adj_mat[i][j] == '1' {
                let mut intersection = graph[i].clone();
                intersection.intersect_with(&graph[j]);
                ans += intersection.count_ones(j + 1..);
            }
        }
    }

    println!("{}", ans);
}
