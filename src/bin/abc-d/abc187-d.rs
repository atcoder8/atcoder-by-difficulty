use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by_key(|&(a, b)| Reverse(2 * a + b));

    let sum_a: usize = ab.iter().map(|x| x.0).sum();
    let mut acc = 0;

    let mut ans = 0;
    for &(a, b) in &ab {
        if acc > sum_a {
            break;
        }

        acc += 2 * a + b;
        ans += 1;
    }

    println!("{}", ans);
}
