use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
    }

    let mut ans = 0;
    let mut pool = BTreeSet::new();
    for &p in &pp {
        pool.insert(p);
        ans += pool.range(..p).next().is_none() as usize;
    }

    println!("{}", ans);
}
