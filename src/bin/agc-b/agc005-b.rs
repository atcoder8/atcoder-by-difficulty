use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa:  [usize; n],
    }

    let mut ans = 0;
    let mut set = BTreeSet::<usize>::new();
    for (a, i) in (0..n).sorted_unstable_by_key(|&i| aa[i]).enumerate() {
        let left = set.range(0..i).next_back().map_or(0, |&x| x + 1);
        let right = set.range(i + 1..n).next().map_or(n - 1, |&x| x - 1);
        ans += (a + 1) * (i - left + 1) * (right - i + 1);
        set.insert(i);
    }

    println!("{}", ans);
}
