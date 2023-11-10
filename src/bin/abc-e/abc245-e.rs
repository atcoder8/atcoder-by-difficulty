use std::{cmp::Reverse, collections::BTreeSet};

use itertools::{izip, Itertools};
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; n],
        cc: [usize; m],
        dd: [usize; m],
    }

    let chocolates = izip!(aa, bb)
        .sorted_unstable_by_key(|&(a, _)| Reverse(a))
        .collect_vec();
    let mut boxes = izip!(cc, dd, 0..m)
        .sorted_unstable_by_key(|&(a, _, _)| a)
        .collect_vec();

    let mut set = BTreeSet::new();
    for (a, b) in chocolates {
        while let Some(&(c, d, id)) = boxes.last() {
            if c < a {
                break;
            }

            boxes.pop();
            set.insert((d, id));
        }

        match set.range((b, 0)..).next() {
            Some(&min_d) => {
                set.remove(&min_d);
            }
            None => return false,
        }
    }

    true
}
