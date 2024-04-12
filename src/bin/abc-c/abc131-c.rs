use num::Integer;
use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let calc_divisor_num = |n: usize| n / c + n / d - n / c.lcm(&d);

    let ans = (b - a + 1) - (calc_divisor_num(b) - calc_divisor_num(a - 1));
    println!("{}", ans);
}
