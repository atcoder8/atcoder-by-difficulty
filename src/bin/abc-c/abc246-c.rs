use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k, x): (usize, usize, usize),
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut rem = k;
    for a in aa.iter_mut() {
        let use_num = (*a / x).min(rem);
        *a -= use_num * x;
        rem -= use_num;
    }

    let mut ans = 0;
    for &a in aa.iter().sorted_unstable_by_key(|&a| Reverse(a)) {
        if rem != 0 {
            rem -= 1;
        } else {
            ans += a;
        }
    }

    println!("{}", ans);
}
