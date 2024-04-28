use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        xx: [i64; n],
    }

    let ans = (0..=n - k)
        .map(|left| {
            let x1 = xx[left];
            let x2 = xx[left + k - 1];
            let abs_x1 = x1.abs();
            let abs_x2 = x2.abs();

            if (x1 <= 0) == (x2 <= 0) {
                abs_x1.max(abs_x2)
            } else {
                abs_x1 + abs_x2 + abs_x1.min(abs_x2)
            }
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
