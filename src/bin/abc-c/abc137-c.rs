use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut ss: [Chars; n],
    }

    ss.iter_mut().for_each(|s| s.sort_unstable());
    ss.sort_unstable();

    let ans: usize = ss
        .iter()
        .dedup_with_count()
        .map(|(cnt, _)| cnt * cnt.saturating_sub(1) / 2)
        .sum();
    println!("{}", ans);
}
