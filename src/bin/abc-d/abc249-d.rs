use proconio::input;

const MAX: usize = 200000;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut counts = vec![0; MAX + 1];
    for &a in &aa {
        counts[a] += 1;
    }

    let mut ans = 0_usize;
    for denom in 1..=MAX {
        for numer in (denom..=MAX).step_by(denom) {
            ans += counts[denom] * counts[numer] * counts[numer / denom];
        }
    }
    println!("{}", ans);
}
