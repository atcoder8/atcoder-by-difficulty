use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        numbers: [usize; 3],
    }

    let ans = numbers.iter().unique().count() == 2;
    println!("{}", if ans { "Yes" } else { "No" });
}
