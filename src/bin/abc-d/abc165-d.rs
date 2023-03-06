use proconio::input;

fn main() {
    input! {
        (a, b, n): (usize, usize, usize),
    }

    let ans = a * (b - 1).min(n) / b;
    println!("{}", ans);
}
