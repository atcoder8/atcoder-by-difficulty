use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let ans = (n / k).pow(3)
        + if k % 2 == 0 {
            ((n + k / 2) / k).pow(3)
        } else {
            0
        };
    println!("{}", ans);
}
