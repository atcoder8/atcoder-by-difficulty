use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

const COSTS: [usize; 10] = [std::usize::MAX, 2, 5, 5, 4, 5, 6, 3, 7, 6];

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; m],
    }

    aa.sort_unstable_by_key(|&a| Reverse(a));

    let mut dp = vec![None; n + 1];
    dp[0] = Some(0);
    for match_num in 1..=n {
        for &a in &aa {
            let cost = COSTS[a];

            if cost > match_num {
                continue;
            }

            if let Some(digit_num) = dp[match_num - cost] {
                update_score(&mut dp[match_num], digit_num + 1);
            }
        }
    }

    let mut digits = vec![];
    let mut match_num = n;
    for reduced_digit_num in (0..dp[n].unwrap()).rev() {
        let is_ok = |a: usize| {
            let cost = COSTS[a];

            cost <= match_num && dp[match_num - cost] == Some(reduced_digit_num)
        };

        let d = *aa.iter().find(|&&a| is_ok(a)).unwrap();
        digits.push(d);
        match_num -= COSTS[d];
    }

    println!("{}", digits.iter().join(""));
}

/// Updates the maximum score.
/// If `score` is `None`, always updated to the candidate score.
///
/// # Arguments
///
/// * `score` - Reference variable for the score to be updated.
/// * `cand_score` - Candidate score to update.
pub fn update_score<T>(score: &mut Option<T>, cand_score: T) -> bool
where
    T: PartialOrd,
{
    if score.as_ref().is_some_and(|score| score >= &cand_score) {
        return false;
    }

    *score = Some(cand_score);

    true
}
