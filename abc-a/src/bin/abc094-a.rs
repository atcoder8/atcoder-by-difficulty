use proconio::input;

fn main() {
    input! {
        (a, b, x): (usize, usize, usize),
    }

    println!("{}", if a <= x && x <= a + b { "YES" } else { "NO" });
}
