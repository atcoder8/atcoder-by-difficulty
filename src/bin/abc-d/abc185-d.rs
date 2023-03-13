use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [Usize1; m],
    }

    aa.sort_unstable();
    aa.push(n);

    let mut white = (0..m)
        .filter(|&i| aa[i + 1] - aa[i] >= 2)
        .map(|i| aa[i + 1] - aa[i] - 1)
        .collect_vec();
    if aa[0] != 0 {
        white.insert(0, aa[0]);
    }

    let ans: usize = if let Some(&min) = white.iter().min() {
        white.iter().map(|&x| (x + min - 1) / min).sum()
    } else {
        0
    };
    println!("{}", ans);
}
