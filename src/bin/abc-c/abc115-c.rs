use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut hh: [usize; n],
    }

    hh.sort_unstable();

    let ans = (0..=n - k)
        .map(|left| hh[left + k - 1] - hh[left])
        .min()
        .unwrap();
    println!("{}", ans);
}
