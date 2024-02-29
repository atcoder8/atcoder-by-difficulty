use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let ans = n % 2 == 0 && s[0..n / 2] == s[n / 2..];
    println!("{}", if ans { "Yes" } else { "No" });
}
