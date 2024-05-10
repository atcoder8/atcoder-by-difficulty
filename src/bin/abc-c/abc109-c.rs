use num::Integer;
use proconio::input;

fn main() {
    input! {
        (n, init): (usize, i64),
        xx: [i64; n],
    }

    let ans = xx.iter().fold(0, |acc, x| acc.gcd(&(x - init)));
    println!("{}", ans);
}
