use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [Usize1; n],
        kk: [Usize1; q],
    }

    let bb = (0..n).map(|i| (aa[i] - i, aa[i] + 1)).collect_vec();

    for &k in &kk {
        let idx = bb.upper_bound_by_key(&k, |&b| b.0);

        let ans = if idx == 0 {
            k
        } else {
            bb[idx - 1].1 + k - bb[idx - 1].0
        };
        println!("{}", ans + 1);
    }
}
