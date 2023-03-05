use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        q: usize,
    }

    let mut s: VecDeque<char> = s.iter().cloned().collect();
    let mut rev = false;

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            rev = !rev;
        } else {
            input! {
                (mut f, c): (usize, char),
            }

            if rev {
                f = 3 - f;
            }

            if f == 1 {
                s.push_front(c);
            } else {
                s.push_back(c);
            }
        }
    }

    let ans: String = if rev {
        s.iter().rev().collect()
    } else {
        s.iter().collect()
    };
    println!("{}", ans);
}
