use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut magics = vec![];
    for i in 0..n {
        let (x1, y1) = xy[i];

        for j in 0..n {
            if i == j {
                continue;
            }

            let (x2, y2) = xy[j];

            let dx = x2 - x1;
            let dy = y2 - y1;
            if dx == 0 {
                magics.push((0, dy / dy.abs()));
            } else if dy == 0 {
                magics.push((dx / dx.abs(), 0));
            } else {
                let g = dx.gcd(&dy);
                magics.push((dx / g, dy / g));
            }
        }
    }
    magics.sort_unstable();
    magics.dedup();

    let ans = magics.len();
    println!("{}", ans);
}
