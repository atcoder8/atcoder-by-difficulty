use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let sorted_ai = enumerate(&aa)
        .map(|(i, &a)| (a, i))
        .sorted_unstable()
        .collect_vec();

    let mid = sorted_ai[n / 2];

    let mut level = 0;
    let min_level_pos = enumerate(&aa)
        .map(|(i, &a)| {
            if (a, i) < mid {
                level += 1;
            } else {
                level -= 1;
            }

            level
        })
        .position_min()
        .unwrap();

    let best_k = (min_level_pos + 1) % n;
    let max_score = sorted_ai[n / 2..].iter().map(|x| x.0).sum::<usize>();
    println!("{best_k} {max_score}");
}
