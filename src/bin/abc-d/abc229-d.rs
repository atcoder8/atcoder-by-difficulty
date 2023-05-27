use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut ans = 0;
    let mut queue = VecDeque::new();
    let mut cnt = 0;
    for i in 0..s.len() {
        queue.push_back(s[i]);
        cnt += (s[i] == '.') as usize;

        while cnt > k {
            cnt -= (queue.pop_front().unwrap() == '.') as usize;
        }

        ans = ans.max(queue.len());
    }

    println!("{}", ans);
}
