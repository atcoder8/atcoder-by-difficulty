use itertools::{enumerate, Itertools};
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        ss: [Bytes; n],
    }

    let mut positions_per_number = vec![vec![]; 10];
    for s in &ss {
        for (i, &c) in enumerate(s) {
            positions_per_number[(c - b'0') as usize].push(i);
        }
    }

    let mut ans = 10 * n;
    for target in 0..=9 {
        let elapsed_time = positions_per_number[target]
            .iter()
            .sorted_unstable()
            .dedup_with_count()
            .map(|(cnt, &pos)| 10 * (cnt - 1) + pos)
            .max()
            .unwrap();
        ans = ans.min(elapsed_time);
    }

    println!("{}", ans);
}
