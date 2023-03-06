use std::collections::VecDeque;

use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        k: usize,
    }

    let mut queue: VecDeque<usize> = (1..10).collect();
    let mut cnt = 0;

    loop {
        let cur = queue.pop_front().unwrap();

        cnt += 1;

        if cnt == k {
            return cur;
        }

        let next = 10 * cur + cur % 10;

        if cur % 10 > 0 {
            queue.push_back(next - 1);
        }

        queue.push_back(next);

        if cur % 10 < 9 {
            queue.push_back(next + 1);
        }
    }
}
