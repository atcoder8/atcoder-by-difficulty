use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dd: [usize; n],
    }

    dd.sort_unstable();

    let half = n / 2;
    let ans = dd[half] - dd[half - 1];
    println!("{}", ans);
}
