use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        aa: [usize; 3],
    }

    let mut ai = aa.iter().enumerate().map(|(i, &a)| (a, i)).collect_vec();
    ai.sort_unstable_by_key(|x| Reverse(x.0));

    let mut order = vec![0; 3];
    for i in 0..3 {
        order[ai[i].1] = i + 1;
    }

    for i in 0..3 {
        println!("{}", order[i]);
    }
}
