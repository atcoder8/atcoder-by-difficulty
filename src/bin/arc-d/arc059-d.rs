use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some((l, r)) => println!("{l} {r}"),
        None => println!("-1 -1"),
    }
}

fn solve() -> Option<(usize, usize)> {
    input! {
        s: Chars,
    }

    let n = s.len();

    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            return Some((i + 1, i + 2));
        }
    }

    for i in 0..n - 2 {
        if s[i] == s[i + 2] {
            return Some((i + 1, i + 3));
        }
    }

    None
}
