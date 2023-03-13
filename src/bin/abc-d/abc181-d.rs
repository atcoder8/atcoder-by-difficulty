use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: Chars,
    }

    let digits = s
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    if digits.len() == 1 {
        return digits[0] % 8 == 0;
    }

    if digits.len() == 2 {
        return (10 * digits[0] + digits[1]) % 8 == 0 || (10 * digits[1] + digits[0]) % 8 == 0;
    }

    let mut counts = vec![0_usize; 10];
    for &d in &digits {
        counts[d] += 1;
    }

    for i in (0..1000).step_by(8) {
        let mut counts2 = vec![0_usize; 10];
        for j in 0..3 {
            counts2[i / 10_usize.pow(j) % 10] += 1;
        }

        if (0..10).all(|j| counts[j] >= counts2[j]) {
            return true;
        }
    }

    false
}
