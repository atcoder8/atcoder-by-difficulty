use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut counts = vec![0_usize; n];
    for &a in &aa {
        counts[a] += 1;
    }

    let whole_comb_num: usize = counts.iter().map(|&cnt| cnt * (cnt - 1) / 2).sum();

    for &a in &aa {
        let ans = whole_comb_num - counts[a].saturating_sub(1);
        println!("{}", ans);
    }
}
