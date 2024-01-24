use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut values = BTreeSet::new();
    for i in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                x: usize,
            }

            values.insert((x, i));

            continue;
        }

        input! {
            (x, k): (usize, usize),
        }

        let ans = if qt == 2 {
            values.range(..(x, q)).rev().skip(k - 1).next()
        } else {
            values.range((x, 0)..).skip(k - 1).next()
        }
        .map(|x| x.0);

        match ans {
            Some(ans) => println!("{}", ans),
            None => println!("-1"),
        }
    }
}
