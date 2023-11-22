use fixedbitset::FixedBitSet;
use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

use crate::dijkstra::dijkstra;

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (i, (a, b, c)) in enumerate(abc) {
        graph[a].push((b, c, i));
        graph[b].push((a, c, i));
    }

    let dists = dijkstra(&graph, 0);
    let mut used = FixedBitSet::with_capacity(m);
    for cur in 1..n {
        for &(adj, dist, edge_idx) in &graph[cur] {
            if dists[adj].unwrap() + dist == dists[cur].unwrap() {
                used.insert(edge_idx);
                break;
            }
        }
    }

    println!("{}", (0..m).filter(|&i| used[i]).map(|i| i + 1).join(" "));
}

pub mod dijkstra {
    use std::{cmp::Reverse, collections::BinaryHeap};

    pub trait Weight: Clone + Ord {
        fn zero() -> Self;
        fn add(&self, rhs: &Self) -> Self;
    }

    pub fn dijkstra<W>(graph: &[Vec<(usize, W, usize)>], start: usize) -> Vec<Option<W>>
    where
        W: Weight,
    {
        let n = graph.len();

        let mut costs = vec![None; n];
        costs[start] = Some(W::zero());

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(W::zero()), start));

        while let Some((Reverse(cost), cur)) = heap.pop() {
            if costs[cur].as_ref() != Some(&cost) {
                continue;
            }

            for (next, edge_cost, _) in &graph[cur] {
                let cand_cost = cost.add(edge_cost);
                let next_cost = &mut costs[*next];

                if next_cost.is_none() || &cand_cost < next_cost.as_ref().unwrap() {
                    *next_cost = Some(cand_cost.clone());
                    heap.push((Reverse(cand_cost), *next));
                }
            }
        }

        costs
    }

    macro_rules! impl_for_builtin_integer {
        ($($ty: tt), *) => {
            $(
                impl Weight for $ty {
                    fn zero() -> Self {
                        0
                    }

                    fn add(&self, rhs: &Self) -> Self {
                        self + rhs
                    }
                }
            )*
        };
    }

    impl_for_builtin_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}
