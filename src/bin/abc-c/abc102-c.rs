use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut bb = aa
        .iter()
        .enumerate()
        .map(|(i, &a)| a - i as i64 + 1)
        .collect_vec();
    bb.sort_unstable();

    let med = bb[n / 2];

    let ans: i64 = bb.iter().map(|&b| (b - med).abs()).sum();
    println!("{}", ans);
}
