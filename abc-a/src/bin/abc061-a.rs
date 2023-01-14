use proconio::input;

fn main() {
    input! {
        (a, b, c): (i64, i64, i64),
    }

    println!("{}", if a <= c && c <= b { "Yes" } else { "No" });
}
