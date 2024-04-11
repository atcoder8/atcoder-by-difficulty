use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let sum = aa.iter().sum::<usize>();

    let counts = aa
        .into_iter()
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();

    if counts.len() == m {
        return 0;
    }

    let next_counts = counts.iter().map(|&(cnt, a)| (cnt, a + m)).collect_vec();

    let mut max_sum_remove = 0;
    let mut sum_remove = 0;
    let mut prev_val = 0;

    for (cnt, val) in chain(counts, next_counts) {
        if val != prev_val + 1 {
            sum_remove = 0;
        }

        sum_remove += val % m * cnt;
        max_sum_remove = max_sum_remove.max(sum_remove);

        prev_val = val;
    }

    sum - max_sum_remove
}
