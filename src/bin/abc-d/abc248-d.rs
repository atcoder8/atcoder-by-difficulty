use itertools::enumerate;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
        q: usize,
        queries: [(Usize1, usize, Usize1); q],
    }

    let mut positions_per_value = vec![vec![]; n];
    for (i, &a) in enumerate(&aa) {
        positions_per_value[a].push(i);
    }

    for &(l, r, x) in &queries {
        let positions = &positions_per_value[x];

        let left = positions.lower_bound(&l);
        let right = positions.lower_bound(&r);
        let ans = right - left;
        println!("{}", ans);
    }
}
