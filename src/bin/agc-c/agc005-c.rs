use proconio::input;

fn main() {
    println!("{}", if solve() { "Possible" } else { "Impossible" });
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let max_a = *aa.iter().max().unwrap();
    let mid = (max_a + 1) / 2;
    let mut counts = vec![0; max_a + 1];
    for &a in &aa {
        counts[a] += 1;
    }

    if counts[..mid].iter().any(|&cnt| cnt != 0) || counts[mid + 1..].iter().any(|&cnt| cnt < 2) {
        return false;
    }

    if max_a % 2 == 1 {
        counts[mid] == 2
    } else {
        counts[mid] == 1
    }
}
