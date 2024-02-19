use itertools::{enumerate, iproduct, izip, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, c): (usize, usize),
        cost_mat: [[usize; c]; c],
        color_mat: [[Usize1; n]; n],
    }

    let mut counts_each_rem = vec![vec![0; c]; 3];
    for (row, col) in iproduct!(0..n, 0..n) {
        counts_each_rem[(row + col) % 3][color_mat[row][col]] += 1;
    }

    let calc_cost = |colors: &[usize]| {
        let mut cost = 0;
        for (counts, &to_color) in izip!(&counts_each_rem, colors) {
            for (from_color, &cnt) in enumerate(counts) {
                cost += cost_mat[from_color][to_color] * cnt;
            }
        }

        cost
    };

    let ans = (0..c)
        .permutations(3)
        .map(|colors| calc_cost(&colors))
        .min()
        .unwrap();
    println!("{}", ans);
}
