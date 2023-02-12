use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let ans = n[..3].iter().all_equal() || n[1..].iter().all_equal();
    println!("{}", if ans { "Yes" } else { "No" });
}
