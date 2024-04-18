use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
        mut bc: [(usize, usize); m],
    }

    aa.sort_unstable();

    bc.sort_unstable_by_key(|v| Reverse(v.1));
    let mut idx = 0;
    for &(b, c) in &bc {
        for _ in 0..b {
            if idx == n || c <= aa[idx] {
                break;
            }

            aa[idx] = c;
            idx += 1;
        }
    }

    let ans = aa.iter().sum::<usize>();
    println!("{}", ans);
}
