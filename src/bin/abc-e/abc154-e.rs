use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
        k: usize,
    }

    let digits = n
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let mut less = vec![0; k + 1];
    let mut equal = vec![0; k + 1];
    equal[0] = 1;

    for &d in &digits {
        let mut next_less = vec![0; k + 1];
        let mut next_equal = vec![0; k + 1];

        for non_zero_num in 0..=k {
            next_less[non_zero_num] += less[non_zero_num];
            if d != 0 {
                next_less[non_zero_num] += equal[non_zero_num];
            } else {
                next_equal[non_zero_num] += equal[non_zero_num];
            }
        }

        for non_zero_num in 0..k {
            next_less[non_zero_num + 1] += 9 * less[non_zero_num];
            if d != 0 {
                next_less[non_zero_num + 1] += (d - 1) * equal[non_zero_num];
                next_equal[non_zero_num + 1] += equal[non_zero_num];
            }
        }

        less = next_less;
        equal = next_equal;
    }

    let ans = less[k] + equal[k];
    println!("{}", ans);
}
