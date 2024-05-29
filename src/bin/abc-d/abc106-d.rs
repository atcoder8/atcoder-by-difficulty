use itertools::{iproduct, Itertools};
use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, q): (usize, usize, usize),
        lr: [(Usize1, Usize1); m],
        pq: [(Usize1, Usize1); q],
    }

    let mut acc = Array2::from_elem((n + 1, n + 1), 0_usize);
    for &(l, r) in &lr {
        acc[(l + 1, r + 1)] += 1;
    }
    for (row, col) in iproduct!(1..=n, 1..n) {
        acc[(row, col + 1)] += acc[(row, col)];
    }
    for (row, col) in iproduct!(1..n, 1..=n) {
        acc[(row + 1, col)] += acc[(row, col)];
    }

    let calc_rect_sum = |top: usize, bottom: usize, left: usize, right: usize| {
        let inclusive = acc[(top, left)] + acc[(bottom, right)];
        let exclusive = acc[(top, right)] + acc[(bottom, left)];

        inclusive - exclusive
    };

    let ans = pq
        .iter()
        .map(|&(p, q)| calc_rect_sum(p, q + 1, p, q + 1))
        .join("\n");
    println!("{}", ans);
}
