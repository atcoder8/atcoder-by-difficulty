use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = 4 - 2 * s.iter().filter(|&&c| c == '-').count() as i64;
    println!("{}", ans);
}
