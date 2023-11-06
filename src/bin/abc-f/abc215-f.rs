use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }

    xy.sort_unstable_by_key(|x| x.0);

    let is_ok = |dest: usize| {
        let mut max_dist = 0;
        let mut queue = VecDeque::new();
        let mut min_y = None;
        let mut max_y = None;
        for &(x, y) in &xy {
            while let Some(&(x0, y0)) = queue.front() {
                if x - x0 >= dest {
                    queue.pop_front();

                    if min_y.is_none() || y0 < min_y.unwrap() {
                        min_y = Some(y0);
                    }

                    if max_y.is_none() || y0 > max_y.unwrap() {
                        max_y = Some(y0);
                    }
                } else {
                    break;
                }
            }

            if let Some(min_y) = min_y {
                max_dist = max_dist.max(y.saturating_sub(min_y));
            }

            if let Some(max_y) = max_y {
                max_dist = max_dist.max(max_y.saturating_sub(y));
            }

            queue.push_back((x, y));
        }

        max_dist >= dest
    };

    let mut ok = 0;
    let mut ng = 10_usize.pow(9) + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
