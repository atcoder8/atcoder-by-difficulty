use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let max_a = *aa.iter().max().unwrap();

    let mut counts = vec![0; max_a + 1];
    for &a in &aa {
        counts[a] += 1;
    }

    let mut divisor_counts = vec![0_usize; max_a + 1];
    for (i, &cnt) in counts.iter().enumerate().skip(1) {
        for mul in (1..).take_while(|&mul| mul <= max_a / i) {
            divisor_counts[i * mul] += cnt;
        }
    }

    let ans = aa.iter().filter(|&&a| divisor_counts[a] == 1).count();
    println!("{}", ans);
}
