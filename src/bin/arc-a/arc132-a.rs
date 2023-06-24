use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        rr: [usize; n],
        cc: [usize; n],
        q: usize,
        rc: [(Usize1, Usize1); q],
    }

    let ans: String = rc
        .iter()
        .map(|&(r, c)| if rr[r] + cc[c] >= n + 1 { '#' } else { '.' })
        .collect();
    println!("{}", ans);
}
