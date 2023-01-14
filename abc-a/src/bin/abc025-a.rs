use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        n: usize,
    }

    println!("{}{}", s[(n - 1) / 5], s[(n - 1) % 5]);
}
