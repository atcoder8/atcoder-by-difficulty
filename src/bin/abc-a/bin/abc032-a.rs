use num::Integer;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    }

    let lcm = a.lcm(&b);
    let ans = (n + lcm - 1) / lcm * lcm;

    println!("{}", ans);
}
