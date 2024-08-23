use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: u32,
        mut ss: [usize; 1 << n],
    }

    let raised = 2_usize.pow(n);

    ss.sort_unstable_by_key(|&s| Reverse(s));

    let mut connected_components = BinaryHeap::<(usize, usize)>::from([(2 * raised - 1, 1)]);

    for (_, group) in &ss.iter().group_by(|&s| s) {
        let mut subtrees: Vec<(usize, usize)> = vec![];
        for _ in 0..group.count() {
            let Some(subtree) = connected_components.pop() else {
                return false;
            };

            subtrees.push(subtree);
        }

        for &(mut size, mut root) in &subtrees {
            while root < raised {
                root *= 2;
                size /= 2;

                connected_components.push((size, root + 1));
            }
        }
    }

    true
}
