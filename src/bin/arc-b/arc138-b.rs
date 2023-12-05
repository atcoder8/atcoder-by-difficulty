use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let flip_num = (0..n).take_while(|&i| aa[i] == i % 2).count();
    let ans = aa[flip_num..].iter().dedup().count() <= flip_num;

    println!("{}", if ans { "Yes" } else { "No" });
}
