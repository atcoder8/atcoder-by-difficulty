use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (x1, y1): (i64, i64),
        (x2, y2): (i64, i64),
    }

    let is_ok = |coord: (i64, i64)| {
        calc_sq_dist(coord, (x1, y1)) == 5 && calc_sq_dist(coord, (x2, y2)) == 5
    };

    let ans = iproduct!(x1 - 5..=x1 + 5, y1 - 5..=y1 + 5).any(is_ok);
    println!("{}", if ans { "Yes" } else { "No" });
}

fn calc_sq_dist(coord1: (i64, i64), coord2: (i64, i64)) -> i64 {
    (coord1.0 - coord2.0).pow(2) + (coord1.1 - coord2.1).pow(2)
}
