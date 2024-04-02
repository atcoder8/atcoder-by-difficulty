use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        xy: [(i64, i64); 3],
    }

    let xx = xy.iter().map(|&(x, _y)| x).unique().collect_vec();
    let yy = xy.iter().map(|&(_x, y)| y).unique().collect_vec();

    let corners = iproduct!(xx, yy).find(|coord| !xy.contains(coord)).unwrap();
    println!("{} {}", corners.0, corners.1);
}
