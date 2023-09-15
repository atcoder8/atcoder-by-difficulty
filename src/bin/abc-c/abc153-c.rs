use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        hh: [usize; n],
    }

    let ans: usize = hh.iter().sorted_by_key(|&&h| Reverse(h)).skip(k).sum();
    println!("{}", ans);
}
