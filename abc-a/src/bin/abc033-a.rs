use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let ans = if n.iter().all_equal() {
        "SAME"
    } else {
        "DIFFERENT"
    };
    println!("{}", ans);
}
