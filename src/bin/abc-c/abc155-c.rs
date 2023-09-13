use itertools::{join, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [String; n],
    }

    ss.sort_unstable();

    let mut tt = vec![];
    let mut counts: Vec<usize> = vec![];
    for s in ss {
        if tt.last() == Some(&s) {
            *counts.last_mut().unwrap() += 1;
        } else {
            tt.push(s);
            counts.push(1);
        }
    }

    let max_cnt = *counts.iter().max().unwrap();
    let ans = tt
        .into_iter()
        .zip(counts)
        .filter(|&(_, cnt)| cnt == max_cnt)
        .map(|(t, _)| t)
        .sorted();
    println!("{}", join(ans, "\n"));
}
