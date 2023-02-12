use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        if s.last().unwrap() == &'T' {
            "YES"
        } else {
            "NO"
        }
    );
}
