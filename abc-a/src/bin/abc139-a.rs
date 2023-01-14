use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let ans = s.iter().zip(&t).filter(|(c1, c2)| c1 == c2).count();
    println!("{}", ans);
}
