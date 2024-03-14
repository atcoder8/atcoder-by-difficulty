use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut contain = vec![false; 10];
    for &c in &s {
        contain[c.to_digit(10).unwrap() as usize] = true;
    }

    let ans = (0..10).find(|&i| !contain[i]).unwrap();
    println!("{}", ans);
}
