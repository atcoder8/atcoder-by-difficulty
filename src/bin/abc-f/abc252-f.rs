use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, l): (usize, usize),
        mut aa: [usize; n],
    }

    let rem = l - aa.iter().sum::<usize>();

    let mut ans = 0;
    let mut heap = BinaryHeap::from(aa.iter().map(|&a| Reverse(a)).collect_vec());
    if rem != 0 {
        heap.push(Reverse(rem));
    }
    while heap.len() >= 2 {
        let combined = heap.pop().unwrap().0 + heap.pop().unwrap().0;
        ans += combined;
        heap.push(Reverse(combined));
    }

    println!("{}", ans);
}
