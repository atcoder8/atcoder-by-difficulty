use itertools::{chain, Itertools};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, k): (usize, usize),
        vv: [i64; n],
    }

    let mut ans = 0;
    for left in 0..=n.min(k) {
        for right in 0..=n.min(k) - left {
            let delete_limit = (left + right).min(k - (left + right));
            let jewels = chain(&vv[..left], &vv[n - right..])
                .cloned()
                .sorted_unstable()
                .collect_vec();
            let delete_num = jewels[..].upper_bound(&0).min(delete_limit);
            let max_sum_val = jewels[delete_num..].iter().sum::<i64>();

            ans = ans.max(max_sum_val);
        }
    }

    println!("{}", ans);
}
