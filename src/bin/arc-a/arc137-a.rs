use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        (l, r): (usize, usize),
    }

    let ans = (1..=r - l)
        .rev()
        .find(|&diff| (l..=r - diff).any(|x| x.gcd(&(x + diff)) == 1))
        .unwrap();
    println!("{}", ans);
}
