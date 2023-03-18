use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, q): (usize, usize, usize),
        mut wv: [(usize, usize); n],
        xx: [usize; m],
        lr: [(Usize1, usize); q],
    }

    wv.sort_unstable_by_key(|x| Reverse(x.1));

    for &(l, r) in &lr {
        let mut useable = xx[..l].iter().chain(xx[r..].iter()).cloned().collect_vec();
        useable.sort_unstable();

        let mut ans = 0;

        for &(w, v) in &wv {
            for box_idx in 0..useable.len() {
                if useable[box_idx] >= w {
                    ans += v;
                    useable.remove(box_idx);

                    break;
                }
            }
        }

        println!("{}", ans);
    }
}
