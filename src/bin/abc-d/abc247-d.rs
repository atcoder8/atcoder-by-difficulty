use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                xc: (usize, usize),
            }

            queue.push_back(xc);
        } else {
            input! {
                c: usize,
            }

            let mut ans = 0;
            let mut rem = c;
            while rem > 0 {
                let (x, c) = queue.pop_front().unwrap();
                if rem > c {
                    ans += x * c;
                    rem -= c;
                } else {
                    ans += x * rem;
                    queue.push_front((x, c - rem));
                    rem = 0;
                }
            }

            println!("{}", ans);
        }
    }
}
