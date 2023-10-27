use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, h): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by_key(|x| Reverse(x.1));
    let max_a = ab.iter().map(|x| x.0).max().unwrap();

    let mut ans = (h + max_a - 1) / max_a;
    let mut rem = h;
    for (i, &(_, b)) in ab.iter().enumerate() {
        rem = rem.saturating_sub(b);
        ans = ans.min(i + 1 + (rem + max_a - 1) / max_a);
    }

    println!("{}", ans);
}
