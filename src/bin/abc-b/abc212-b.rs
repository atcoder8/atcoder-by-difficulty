use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let xx = s.iter().map(|c| c.to_digit(10).unwrap()).collect_vec();

    let is_straight = || {
        for i in 0..3 {
            if (xx[i] + 1) % 10 != xx[i + 1] {
                return false;
            }
        }

        true
    };

    let ans = if xx.iter().all_equal() || is_straight() {
        "Weak"
    } else {
        "Strong"
    };
    println!("{}", ans);
}
