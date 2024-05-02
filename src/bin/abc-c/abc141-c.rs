use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k, q): (usize, usize, usize),
        aa: [Usize1; q],
    }

    let mut score = vec![k; n];
    for &a in &aa {
        score[a] += 1;
    }

    let ans = score
        .iter()
        .map(|&score| if score > q { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}
