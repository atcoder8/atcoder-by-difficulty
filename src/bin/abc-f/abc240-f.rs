use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> i64 {
    input! {
        (n, _m): (usize, usize),
        xy: [(i64, i64); n],
    }

    let mut ans = xy[0].0;
    let mut b = 0;
    let mut sum = 0;
    for &(x, y) in &xy {
        let positive_b_num = if x >= 0 {
            y
        } else if b >= 0 {
            (b / -x).min(y)
        } else {
            0
        };
        if positive_b_num != 0 {
            ans = ans.max(sum + b * positive_b_num + x * positive_b_num * (positive_b_num + 1) / 2);
        }

        sum += b * y + x * y * (y + 1) / 2;
        b += x * y;
    }

    ans
}
