use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut front = vec![];
    let mut back = vec![];
    for (i, a) in enumerate(aa) {
        if i % 2 == 0 {
            back.push(a);
        } else {
            front.push(a);
        }
    }

    let mut ans = front.into_iter().rev().chain(back).collect_vec();
    if n % 2 == 1 {
        ans.reverse();
    }
    println!("{}", ans.iter().join(" "));
}
