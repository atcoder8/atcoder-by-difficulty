use proconio::input;

fn main() {
    input! {
        n: usize,
        (x0, y0): (f64, f64),
        (x_half, y_half): (f64, f64),
    }

    let center_x = (x0 + x_half) / 2.0;
    let center_y = (y0 + y_half) / 2.0;

    let diff_x = x0 - center_x;
    let diff_y = y0 - center_y;

    let radius = (diff_x.powi(2) + diff_y.powi(2)).sqrt();
    let theta = (diff_y).atan2(diff_x);

    let radian = theta + 2.0 * std::f64::consts::PI / n as f64;
    let (sin, cos) = radian.sin_cos();

    let (x1, y1) = (center_x + radius * cos, center_y + radius * sin);
    println!("{} {}", x1, y1);
}
