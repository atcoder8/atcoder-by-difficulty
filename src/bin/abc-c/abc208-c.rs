use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [Usize1; n],
    }

    let orders = (0..n).sorted_by_key(|&i| aa[i]).collect_vec();
    let (q, r) = (k / n, k % n);
    let mut answers = vec![q; n];
    for &order in orders.iter().take(r) {
        answers[order] += 1;
    }

    for ans in answers {
        println!("{}", ans);
    }
}
