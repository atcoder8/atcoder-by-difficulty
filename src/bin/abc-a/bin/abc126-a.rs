use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (_n, k): (usize, Usize1),
        mut s: Chars,
    }

    s[k] = s[k].to_ascii_lowercase();

    let ans: String = s.iter().collect();
    println!("{}", ans);
}
