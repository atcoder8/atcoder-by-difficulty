use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s: Chars,
        i: Usize1,
    }

    println!("{}", s[i]);
}
