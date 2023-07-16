use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let mut min = s.clone();
    let mut max = s.clone();

    for _ in 0..s.len() {
        s.rotate_left(1);
        min = min.min(s.clone());
        max = max.max(s.clone());
    }

    let min: String = min.iter().collect();
    let max: String = max.iter().collect();

    println!("{}\n{}", min, max);
}
