use proconio::input;

fn main() {
    input! {
        (w, h, x, y): (usize, usize, usize, usize),
    }

    let ans1 = (w * h) as f64 / 2.0;
    let ans2 = (2 * x == w && 2 * y == h) as usize;
    println!("{ans1} {ans2}");
}
