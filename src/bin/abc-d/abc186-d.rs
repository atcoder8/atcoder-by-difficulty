use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [i64; n],
    }

    aa.sort_unstable_by_key(|&a| Reverse(a));

    let mut acc_a = 0;
    let mut ans = 0;
    for (i, &a) in aa.iter().enumerate() {
        ans += acc_a - a * i as i64;
        acc_a += a;
    }

    println!("{}", ans);
}
