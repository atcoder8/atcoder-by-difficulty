use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let bad = s.iter().find_position(|&&c| c == '1').unwrap().0;
    println!("{}", if bad % 2 == 0 { "Takahashi" } else { "Aoki" });
}
