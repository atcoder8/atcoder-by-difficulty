use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable_by_key(|&a| Reverse(a));

    let ans =
        aa[0] + 2 * aa[1..(n / 2)].iter().sum::<usize>() + if n % 2 == 1 { aa[n / 2] } else { 0 };
    println!("{}", ans);
}
