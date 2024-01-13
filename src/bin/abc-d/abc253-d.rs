use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    let lcm = a.lcm(&b);

    let ans = calc_sum(n) - (a * calc_sum(n / a) + b * calc_sum(n / b) - lcm * calc_sum(n / lcm));
    println!("{}", ans);
}

fn calc_sum(n: usize) -> usize {
    n * (n + 1) / 2
}
