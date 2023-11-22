use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (_n, m): (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut ans = 0;
    let mut most_right = 0;
    for &(a, b) in ab.iter().sorted_unstable_by_key(|x| x.1) {
        if a >= most_right {
            ans += 1;
            most_right = b;
        }
    }

    println!("{}", ans);
}
