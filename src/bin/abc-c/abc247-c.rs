use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut s = vec![];
    for i in 1..=n {
        let mut next_s = s.clone();
        next_s.push(i);
        next_s.extend(s);
        s = next_s;
    }

    println!("{}", s.iter().join(" "));
}
