use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [i64; n + 1],
        mut cc: [i64; n + m + 1],
    }

    let mut bb = vec![0; m + 1];
    for i in (0..=m).rev() {
        bb[i] = cc[n + i] / aa[n];
        for j in 0..=n {
            cc[i + j] -= bb[i] * aa[j];
        }
    }

    println!("{}", bb.iter().join(" "));
}
