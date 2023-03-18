use itertools::join;
use proconio::input;

const MOD: usize = 200;

fn main() {
    if let Some((bb, cc)) = solve() {
        println!("Yes");
        println!("{} {}", bb.len(), join(bb, " "));
        println!("{} {}", cc.len(), join(cc, " "));
    } else {
        println!("No");
    }
}

fn solve() -> Option<(Vec<usize>, Vec<usize>)> {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.iter_mut().for_each(|a| *a %= MOD);

    let reconstruct = |dp: &Vec<Vec<Option<usize>>>, last_val: usize, last_idx: usize| {
        let mut seq = vec![];
        let mut cur_val = last_val;
        let mut cur_idx = dp[last_idx][last_val];

        while let Some(idx) = cur_idx {
            seq.push(idx + 1);
            let prev_val = (cur_val + MOD - aa[idx]) % MOD;
            let prev_idx = dp[idx][prev_val];
            cur_val = prev_val;
            cur_idx = prev_idx;
        }

        seq.reverse();

        seq
    };

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; MOD]];

    for (elem_idx, &a) in aa.iter().enumerate() {
        dp.push(dp[elem_idx].clone());

        for prev_val in 0..MOD {
            if dp[elem_idx][prev_val].is_some() {
                let cur_val = (prev_val + a) % MOD;

                dp[elem_idx + 1][cur_val] = Some(elem_idx);

                if dp[elem_idx][cur_val].is_some() {
                    let bb = reconstruct(&dp, cur_val, elem_idx);
                    let cc = reconstruct(&dp, cur_val, elem_idx + 1);

                    return Some((bb, cc));
                }
            }
        }

        if dp[elem_idx + 1][a].is_some() {
            let bb = reconstruct(&dp, a, elem_idx + 1);
            let cc = vec![elem_idx + 1];

            return Some((bb, cc));
        }

        dp[elem_idx + 1][a] = Some(elem_idx);
    }

    None
}
