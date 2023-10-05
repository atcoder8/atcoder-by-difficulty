use proconio::input;

fn main() {
    input! {
        (x0, y0): (f64, f64),
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut ans = 200.0;
    for i in 0..n {
        let (x1, y1) = xy[i];
        let (x2, y2) = xy[(i + 1) % n];
        let (diff_x, diff_y) = (x1 - x2, y1 - y2);

        let dist = if diff_x.abs() <= 1e-6 {
            (x0 - x1).abs()
        } else {
            let (a, b, c) = (diff_y, -diff_x, diff_x * y1 - diff_y * x1);
            (a * x0 + b * y0 + c).abs() / (a.powi(2) + b.powi(2)).sqrt()
        };

        if dist < ans {
            ans = dist;
        }
    }

    println!("{}", ans);
}
