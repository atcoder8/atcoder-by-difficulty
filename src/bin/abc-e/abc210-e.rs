use itertools::Itertools;
use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut ac: [(usize, usize); m],
    }

    let mut ans = 0;
    let mut gcd = n;
    for &(a, c) in ac.iter().sorted_unstable_by_key(|x| x.1) {
        let next_gcd = gcd.gcd(&a);
        ans += c * (gcd - next_gcd);
        gcd = next_gcd;
    }

    if gcd == 1 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
