use proconio::input;

fn main() {
    let (min_dist, max_dist) = solve();
    println!("{max_dist}\n{min_dist}");
}

fn solve() -> (usize, usize) {
    input! {
        n: usize,
        dd: [usize; n],
    }

    let sum_d: usize = dd.iter().sum();

    let min_dist = match n {
        1 => dd[0],
        2 => dd[0].abs_diff(dd[1]),
        _ => {
            let max_d = *dd.iter().max().unwrap();
            if sum_d >= max_d * 2 {
                0
            } else {
                2 * max_d - sum_d
            }
        }
    };

    (min_dist, sum_d)
}
