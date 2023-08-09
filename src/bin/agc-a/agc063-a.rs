use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut a_queue = VecDeque::new();
    let mut b_queue = VecDeque::new();
    for (i, &c) in s.iter().enumerate() {
        if c == 'A' {
            a_queue.push_back(i);
        } else {
            b_queue.push_back(i);
        }
    }

    for i in 1..=n {
        if i % 2 == 1 {
            b_queue.pop_front();
        } else {
            a_queue.pop_front();
        }

        let winner = match (a_queue.front(), b_queue.front()) {
            (None, None) => unreachable!(),
            (None, Some(_)) => "Bob",
            (Some(_), None) => "Alice",
            (Some(a), Some(b)) => {
                if a < b {
                    "Alice"
                } else {
                    "Bob"
                }
            }
        };
        println!("{}", winner);
    }
}
