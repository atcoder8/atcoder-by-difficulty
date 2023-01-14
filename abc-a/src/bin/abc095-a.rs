use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = 700 + 100 * s.iter().filter(|&&c| c == 'o').count();
    println!("{}", ans);
}
