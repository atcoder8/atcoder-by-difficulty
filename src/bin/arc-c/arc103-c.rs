use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        vv: [usize; n],
    }

    let counts1 = vv
        .iter()
        .step_by(2)
        .sorted_unstable()
        .dedup_with_count()
        .sorted_unstable_by_key(|x| Reverse(x.0))
        .chain([(0, &0)])
        .collect_vec();

    let counts2 = vv[1..]
        .iter()
        .step_by(2)
        .sorted_unstable()
        .dedup_with_count()
        .sorted_unstable_by_key(|x| Reverse(x.0))
        .chain([(0, &0)])
        .collect_vec();

    let ans = n - if counts1[0].1 == counts2[0].1 {
        (counts1[0].0 + counts2[1].0).max(counts1[1].0 + counts2[0].0)
    } else {
        counts1[0].0 + counts2[0].0
    };

    println!("{}", ans);
}
