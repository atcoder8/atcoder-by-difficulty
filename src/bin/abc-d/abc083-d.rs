use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let ans = (1..n)
        .filter(|&i| s[i - 1] != s[i])
        .map(|i| i.max(n - i))
        .min()
        .unwrap_or(n);
    println!("{}", ans);
}
