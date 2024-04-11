use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        _aa: [usize; n],
    }

    let ans = (n - 1 + (k - 2)) / (k - 1);
    println!("{}", ans);
}
