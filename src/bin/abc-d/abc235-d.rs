use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        (a, n): (usize, usize),
    }

    let mut dists = vec![10 * n; 10 * n];
    dists[1] = 0;
    let mut queue = VecDeque::from(vec![1]);
    while let Some(cur) = queue.pop_front() {
        let candidate_dist = dists[cur] + 1;

        let next = a * cur;
        if next < 10 * n && candidate_dist < dists[next] {
            dists[next] = candidate_dist;
            queue.push_back(next);
        }

        let exp = (0..).find(|&i| 10_usize.pow(i) > cur / 10).unwrap();
        let next = cur / 10 + cur % 10 * 10_usize.pow(exp) as usize;
        if cur >= 10 && cur % 10 != 0 {
            if next < 10 * n && candidate_dist < dists[next] {
                dists[next] = candidate_dist;
                queue.push_back(next);
            }
        }
    }

    if dists[n] != 10 * n {
        println!("{}", dists[n]);
    } else {
        println!("-1");
    }
}
