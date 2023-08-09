use proconio::input;

fn main() {
    input! {
        (x1, y1): (f64, f64),
        (x2, y2): (f64, f64),
        (x3, y3): (f64, f64),
    }

    let (a, b) = (x2 - x1, y2 - y1);
    let (c, d) = (x3 - x1, y3 - y1);
    let ans = (a * d - b * c).abs() / 2.0;

    println!("{}", ans);
}
