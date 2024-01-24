use itertools::iproduct;
use proconio::{input, marker::Chars};

const LEN: usize = 6;

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let check_hor = |(row, col): (usize, usize)| {
        col + LEN <= n && (0..LEN).filter(|&j| ss[row][col + j] == '.').count() <= 2
    };

    let check_ver = |(row, col): (usize, usize)| {
        row + LEN <= n && (0..LEN).filter(|&i| ss[row + i][col] == '.').count() <= 2
    };

    let check_down_right = |(row, col): (usize, usize)| {
        row + LEN <= n
            && col + LEN <= n
            && (0..LEN).filter(|&i| ss[row + i][col + i] == '.').count() <= 2
    };

    let check_up_right = |(row, col): (usize, usize)| {
        row >= LEN - 1
            && col + LEN <= n
            && (0..LEN).filter(|&i| ss[row - i][col + i] == '.').count() <= 2
    };

    let check_match = |coord: (usize, usize)| {
        check_hor(coord) || check_ver(coord) || check_down_right(coord) || check_up_right(coord)
    };

    let ans = iproduct!(0..n, 0..n).any(check_match);
    println!("{}", if ans { "Yes" } else { "No" });
}
