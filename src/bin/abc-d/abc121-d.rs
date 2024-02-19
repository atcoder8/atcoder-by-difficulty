use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let calc_xor = |n: usize| (n + 1) / 2 % 2 ^ n * (n % 2 == 0) as usize;

    let ans = calc_xor(b) ^ calc_xor(a.saturating_sub(1));
    println!("{}", ans);
}
