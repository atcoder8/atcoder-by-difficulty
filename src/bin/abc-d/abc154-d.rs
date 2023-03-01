use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        pp: [usize; n],
    }

    let calc_expected_value = |p: usize| (p + 1) as f64 / 2.0;

    let mut sum_expected_value: f64 = pp.iter().take(k).map(|&p| calc_expected_value(p)).sum();

    let mut ans = sum_expected_value;

    for i in 0..(n - k) {
        sum_expected_value += calc_expected_value(pp[i + k]) - calc_expected_value(pp[i]);
        ans = ans.max(sum_expected_value);
    }

    println!("{}", ans);
}
