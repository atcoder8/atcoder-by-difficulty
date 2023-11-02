use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, q): (usize, usize),
        s: Chars,
        lr: [(Usize1, usize); q],
    }

    let mut counts = vec![0; n + 1];
    for (i, win) in s.windows(2).enumerate() {
        if win == &['A', 'C'] {
            counts[i + 1] += 1;
        }
    }
    for i in 1..n {
        counts[i + 1] += counts[i];
    }

    for &(l, r) in &lr {
        let ans = counts[r - (s[r - 1] == 'A') as usize] - counts[l];
        println!("{}", ans);
    }
}
