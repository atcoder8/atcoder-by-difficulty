use itertools::Itertools;
use proconio::input;

const LIMIT: usize = 10000;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![6, 10, 15, 12];
    for i in 18..=LIMIT {
        if i % 6 == 0 || i % 10 == 0 || i % 15 == 0 {
            ans.push(i);
        }
    }

    println!("{}", ans.iter().take(n).join(" "));
}
