use num::Integer;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let mut acc_lcm = 1;

    for &a in &aa {
        acc_lcm = acc_lcm.lcm(&a);

        if acc_lcm > 2 * m {
            return 0;
        }
    }

    if aa.iter().any(|&a| acc_lcm / 2 % a == 0) {
        return 0;
    }

    (m + acc_lcm / 2) / acc_lcm
}
