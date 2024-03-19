use std::cmp::Reverse;

use itertools::{chain, enumerate, iproduct, Itertools};
use ndarray::Array2;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [String; n],
    }

    ss.sort_unstable_by_key(|s| Reverse(s.len()));

    let mut shrink_strings: Vec<String> = vec![];
    for cand_s in ss {
        if shrink_strings.iter().all(|s| !s.contains(&cand_s)) {
            shrink_strings.push(cand_s);
        }
    }

    let str_num = shrink_strings.len();
    let sum_len = shrink_strings.iter().map(String::len).sum::<usize>();

    // `overlap_mat[(i, j)]`: `shrink_strings[i]`の後に`shrink_strings[j]`を繋げるときに重ね合わせることができる最大の長さ
    let mut overlap_mat = Array2::from_elem((str_num, str_num), 0_usize);
    for (i, j) in iproduct!(0..str_num, 0..str_num) {
        let prev = &shrink_strings[i];
        let next = &shrink_strings[j];

        let concat = chain(next.chars(), prev.chars()).collect_vec();
        let lengths = z_algorithm(&concat);

        let overlap_len = (1..=next.len().min(prev.len()))
            .rev()
            .find(|&i| lengths[concat.len() - i] == i)
            .unwrap_or(0);
        overlap_mat[(i, j)] = overlap_len;
    }

    let mut dp = vec![vec![0; str_num]; 1 << str_num];
    for (i, s) in enumerate(&shrink_strings) {
        dp[0][i] = s.len();
    }
    for bit in 1..1 << str_num {
        for (prev, next) in iproduct!(0..str_num, 0..str_num) {
            if bit >> prev & 1 == 1 && bit >> next & 1 == 0 {
                chmax!(
                    dp[bit | 1 << next][next],
                    dp[bit][prev] + overlap_mat[(prev, next)]
                );
            }
        }
    }

    let max_overlap_len = *dp[(1 << str_num) - 1].iter().max().unwrap();
    let ans = sum_len - max_overlap_len;
    println!("{}", ans);
}

/// For each non-negative integer `i` less than `|seq|`,
/// find the length of the longest common prefix of `seq` and `seq[i..]`.
pub fn z_algorithm<T>(seq: &[T]) -> Vec<usize>
where
    T: Eq,
{
    if seq.is_empty() {
        return vec![];
    }

    let n = seq.len();

    let mut lengths = vec![0; n];
    lengths[0] = n;

    let mut cursor = 1;
    let mut common_len = 0;
    while cursor < n {
        while cursor + common_len < n && seq[cursor + common_len] == seq[common_len] {
            common_len += 1;
        }

        if common_len == 0 {
            cursor += 1;
            continue;
        }

        lengths[cursor] = common_len;

        let mut shift = 1;
        while shift + lengths[shift] < common_len {
            lengths[cursor + shift] = lengths[shift];
            shift += 1;
        }

        cursor += shift;
        common_len -= shift;
    }

    lengths
}

/// If the right-hand side is greater than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is less than or equal to the left-hand side,
/// does nothing and returns `false`.
///
/// The left-hand and right-hand sides must be the same type
/// and must implement the `Clone` trait.
///
/// # Examples
///
/// ```
/// let mut x = 5;
///
/// assert_eq!(chmax!(x, 3), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmax!(x, 7), true);
/// assert_eq!(x, 7);
/// ```
#[macro_export]
macro_rules! chmax {
    ($lhs: expr, $rhs: expr) => {
        if $rhs > $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
