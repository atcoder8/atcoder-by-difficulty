use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        aa: [usize; n],
    }

    let mut ans = 0;
    let mut max_y_idx: Option<usize> = None;
    let mut max_x_idx: Option<usize> = None;
    let mut border = 0;
    for (i, &a) in aa.iter().enumerate() {
        if a < y || a > x {
            max_y_idx = None;
            max_x_idx = None;
            border = i + 1;

            continue;
        }

        if a == y {
            max_y_idx = Some(i);
        }

        if a == x {
            max_x_idx = Some(i);
        }

        if let (Some(max_y_idx), Some(max_x_idx)) = (max_y_idx, max_x_idx) {
            ans += max_y_idx.min(max_x_idx) - border + 1;
        }
    }

    println!("{}", ans);
}
