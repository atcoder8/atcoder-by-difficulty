use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let ans: String = n
        .iter()
        .map(|&c| if c == '1' { '9' } else { '1' })
        .collect();
    println!("{}", ans);
}
