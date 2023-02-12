use num::Integer;
use proconio::input;

fn main() {
    input! {
        (w, h): (usize, usize),
    }

    let gcd = w.gcd(&h);

    println!("{}:{}", w / gcd, h / gcd);
}
