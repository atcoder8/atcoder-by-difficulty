use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        t: usize,
        slr: [(Bytes, usize, usize); t],
    }

    for (s, l, r) in slr {
        let s = s.iter().map(|&c| (c - b'0') as usize).collect_vec();
        let ans = solve(&s, r) - solve(&s, l - 1);
        println!("{}", ans);
    }
}

fn to_vec(mut x: usize) -> Vec<usize> {
    let mut v = vec![];
    while x != 0 {
        v.push(x % 10);
        x /= 10;
    }
    v.reverse();

    v
}

fn solve(s: &[usize], x: usize) -> usize {
    if x == 0 {
        return 0;
    }

    let s_len = s.len();
    let digits = to_vec(x);
    let n = digits.len();

    let mut less = vec![0_usize; s_len + 1];
    let mut equal = vec![0_usize; s_len + 1];
    less[0] = digits[0].saturating_sub(1);
    less[1] = (s[0] != 0 && s[0] < digits[0]) as usize;
    equal[0] = (digits[0] != 0) as usize;
    equal[1] = (s[0] == digits[0]) as usize;
    let mut ans = less[s_len] * 10_usize.pow((n - 1) as u32)
        + equal[s_len] * (x % 10_usize.pow((n - 1) as u32) + 1);

    for (digit_idx, &d) in digits.iter().enumerate().skip(1) {
        let mut next_less = vec![0; s_len + 1];
        let mut next_equal = vec![0; s_len + 1];

        // {1, 2, ... , 9} -> less
        next_less[0] += 9;
        next_less[1] += (s[0] != 0) as usize;

        // less -> less
        next_less[0] += 10 * less[0];
        for add in 0..10 {
            for i in 0..s_len {
                if s[i] == add {
                    next_less[i + 1] += less[i];
                }
            }
        }

        // equal -> less
        next_less[0] += d * equal[0];
        for add in 0..d {
            for i in 0..s_len {
                if s[i] == add {
                    next_less[i + 1] += equal[i];
                }
            }
        }

        // equal -> equal
        next_equal[0] += equal[0];
        for i in 0..s_len {
            if s[i] == d {
                next_equal[i + 1] += equal[i];
            }
        }

        less = next_less;
        equal = next_equal;

        ans += less[s_len] * 10_usize.pow((n - 1 - digit_idx) as u32)
            + equal[s_len] * (x % 10_usize.pow((n - 1 - digit_idx) as u32) + 1);
    }

    ans
}
