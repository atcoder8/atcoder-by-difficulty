use itertools::Itertools;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        cc: [u8; n],
    }

    if cc.iter().all_equal() {
        return None;
    }

    let max_len = cc
        .iter()
        .chain(&cc)
        .dedup_with_count()
        .map(|(cnt, _)| cnt)
        .max()
        .unwrap();

    Some((max_len + 1) / 2)
}
