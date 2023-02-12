use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = s[0] == s[1] || s[1] == s[2] || s[2] == s[3];
    println!("{}", if ans { "Bad" } else { "Good" });
}
