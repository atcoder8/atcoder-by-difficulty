use itertools::iproduct;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("Infinity"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, k): (usize, usize),
        mut xy: [(i64, i64); n],
    }

    if k == 1 {
        return None;
    }

    let mut ans = 0;
    let mut used = vec![vec![false; n]; n];
    for i in 0..n {
        let (x1, y1) = xy[i];

        for j in i + 1..n {
            let (x2, y2) = xy[j];

            if used[i][j] {
                continue;
            }

            let (diff_x_1, diff_y_1) = (x2 - x1, y2 - y1);

            let mut cnt = 2;
            let mut points = vec![i, j];
            for k in j + 1..n {
                let (x3, y3) = xy[k];
                let (diff_x_2, diff_y_2) = (x3 - x1, y3 - y1);

                if diff_x_1 * diff_y_2 == diff_x_2 * diff_y_1 {
                    cnt += 1;
                    points.push(k);
                }
            }

            if cnt >= k {
                ans += 1;
            }

            for (&p1, &p2) in iproduct!(&points, &points) {
                used[p1][p2] = true;
            }
        }
    }

    Some(ans)
}
