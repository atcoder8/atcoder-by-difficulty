use proconio::{input, marker::Chars};

fn main() {
    input! {
        (_, s, _): (String, Chars, String),
    }

    println!("A{}C", s[0]);
}
