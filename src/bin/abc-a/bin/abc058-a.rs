use proconio::input;

fn main() {
    input! {
        (a, b, c): (i64, i64, i64),
    }

    println!("{}", if b - a == c - b { "YES" } else { "NO" });
}
