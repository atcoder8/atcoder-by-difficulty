use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        mut vv: [usize; n],
    }

    vv.sort_unstable_by_key(|&v| Reverse(v));

    let sum: usize = vv[..a].iter().sum();
    let avg = sum as f64 / a as f64;

    let min = vv[a - 1];
    let right = vv.partition_point(|&v| v >= min);
    let comb_num = if vv[0] == min {
        (a..=b).map(|k| calc_combinations(right, k)).sum()
    } else {
        let left = vv.partition_point(|&v| v > min);

        calc_combinations(right - left, a - left)
    };

    println!("{avg}\n{comb_num}");
}

/// Calculates the number of combinations that select k elements from n elements.
pub fn calc_combinations(n: usize, r: usize) -> usize {
    if r > n {
        return 0;
    }

    let r = r.min(n - r);
    let mut ret = 1;
    for i in 0..r {
        ret *= n - i;
        ret /= i + 1;
    }

    ret
}
