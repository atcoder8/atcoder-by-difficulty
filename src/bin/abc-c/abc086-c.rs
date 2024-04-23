use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        txy: [(usize, usize, usize); n],
    }

    let ans =
        [(0, 0, 0)]
            .iter()
            .chain(&txy)
            .tuple_windows()
            .all(|(&(t1, x1, y1), &(t2, x2, y2))| {
                let duration = t2 - t1;
                let dist = calc_dist((x1, y1), (x2, y2));

                duration >= dist && duration % 2 == dist % 2
            });
    println!("{}", if ans { "Yes" } else { "No" });
}

fn calc_dist(coord1: (usize, usize), coord2: (usize, usize)) -> usize {
    coord1.0.abs_diff(coord2.0) + coord1.1.abs_diff(coord2.1)
}
