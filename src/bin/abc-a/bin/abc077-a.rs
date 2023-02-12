use proconio::{input, marker::Chars};

fn main() {
    input! {
        ccc: [Chars; 2],
    }

    let ans = (0..3).all(|i| ccc[0][i] == ccc[1][2 - i]);
    println!("{}", if ans { "YES" } else { "NO" });
}
