use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    }

    let mut counts = vec![0_usize; 100_001];
    for &a in &aa {
        counts[a] += 1;
    }

    let mut s: usize = aa.iter().sum();

    for &(b, c) in &bc {
        s -= b * counts[b];
        s += c * counts[b];
        counts[c] += counts[b];
        counts[b] = 0;

        println!("{}", s);
    }
}
