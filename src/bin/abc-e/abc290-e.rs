use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let seq = (0..n)
        .sorted_by_key(|&i| Reverse(i.min(n - 1 - i)))
        .collect_vec();
    let mut counts = vec![0; n];
    let mut ans = 0;
    for (ord, &idx) in seq.iter().enumerate() {
        let a = aa[idx];
        ans += (idx + 1).min(n - idx) * (ord - counts[a]);
        counts[a] += 1;
    }

    println!("{}", ans);
}
