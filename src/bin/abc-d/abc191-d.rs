use proconio::input;

const SCALE: i64 = 10000;

fn main() {
    input! {
        (x, y, r): (f64, f64, f64),
    }

    let (x, y, r) = (to_i64(x), to_i64(y), to_i64(r));

    let mut ans = 0;

    let min_x = ceil_div(x - r, SCALE);
    let max_x = floor_div(x + r, SCALE);

    for x0 in min_x..=max_x {
        let scaled_x0 = x0 * SCALE;

        let diff_x = (x - scaled_x0).abs();

        let sq_diff_y = r * r - diff_x * diff_x;

        let low_lim = ceil_div(-(calc_floor_sqrt(sq_diff_y as usize) as i64) + y, SCALE);
        let upp_lim = floor_div(calc_floor_sqrt(sq_diff_y as usize) as i64 + y, SCALE);

        ans += upp_lim - low_lim + 1;
    }

    println!("{}", ans);
}

fn to_i64(x: f64) -> i64 {
    (SCALE as f64 * x).round() as i64
}

pub fn floor_div(a: i64, b: i64) -> i64 {
    let abs_a = a.abs();
    let abs_b = b.abs();

    if a.is_positive() == b.is_positive() {
        abs_a / abs_b
    } else {
        -((abs_a + abs_b - 1) / abs_b)
    }
}

pub fn ceil_div(a: i64, b: i64) -> i64 {
    let abs_a = a.abs();
    let abs_b = b.abs();

    if a.is_positive() == b.is_positive() {
        (abs_a + abs_b - 1) / abs_b
    } else {
        -(abs_a / abs_b)
    }
}

pub fn calc_floor_sqrt(n: usize) -> usize {
    let mut ng = 1_usize << 0_usize.count_zeros() / 2;
    let mut ok = 0;

    while ng - ok >= 2 {
        let mid = (ng + ok) / 2;

        if mid * mid <= n {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}

pub fn calc_ceil_sqrt(n: usize) -> usize {
    let floor_sqrt_n = calc_floor_sqrt(n);

    if floor_sqrt_n * floor_sqrt_n < n {
        floor_sqrt_n + 1
    } else {
        floor_sqrt_n
    }
}
