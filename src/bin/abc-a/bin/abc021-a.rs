use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", n);
    println!("{}", join((0..n).map(|_| 1), "\n"));
}
