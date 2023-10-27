use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        xx: [Usize1; q],
    }

    let mut idx_to_aa = (0..n).collect_vec();
    let mut aa_to_idx = (0..n).collect_vec();
    for &val1 in &xx {
        let idx1 = aa_to_idx[val1];
        let idx2 = if idx1 < n - 1 { idx1 + 1 } else { idx1 - 1 };
        let val2 = idx_to_aa[idx2];
        idx_to_aa.swap(idx1, idx2);
        aa_to_idx.swap(val1, val2);
    }

    println!("{}", idx_to_aa.iter().map(|&a| a + 1).join(" "));
}
