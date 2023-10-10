use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        (q, a, b): (usize, i64, i64),
        tab: [(usize, i64, i64); q],
    }

    let mut set = BTreeSet::from([a - b, a + b]);
    for (t, a, b) in tab {
        if t == 1 {
            set.insert(a - b);
            set.insert(a + b);
        } else {
            if set.range(a..=b).next().is_some() {
                println!("0");
                continue;
            }

            let mut ans = None;
            if let Some(&left) = set.range(..a).next_back() {
                ans = Some(a - left);
            }
            if let Some(&right) = set.range((b + 1)..).next() {
                if ans.is_none() || right - b < ans.unwrap() {
                    ans = Some(right - b);
                }
            }

            println!("{}", ans.unwrap());
        }
    }
}
