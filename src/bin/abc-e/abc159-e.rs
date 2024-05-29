use itertools::{chain, iproduct, Itertools};
use ndarray::Array2;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w, k): (usize, usize, usize),
        sss: [Chars; h],
    }

    let mut acc = Array2::from_shape_fn((h + 1, w + 1), |(row, col)| {
        (row > 0 && col > 0 && sss[row - 1][col - 1] == '1') as usize
    });
    for (row, col) in iproduct!(1..=h, 1..w) {
        acc[(row, col + 1)] += acc[(row, col)];
    }
    for (row, col) in iproduct!(1..h, 1..=w) {
        acc[(row + 1, col)] += acc[(row, col)];
    }

    let calc_white_num = |top: usize, bottom: usize, left: usize, right: usize| {
        let inclusive = acc[(top, left)] + acc[(bottom, right)];
        let exclusive = acc[(top, right)] + acc[(bottom, left)];

        inclusive - exclusive
    };

    let count_ver_div = |bounds: &[usize]| {
        let mut div_cnt = 0;

        let mut left = 0_usize;
        loop {
            let mut ok = left;
            let mut ng = w + 1;

            while ok.abs_diff(ng) > 1 {
                let mid = (ok + ng) / 2;

                if bounds
                    .iter()
                    .tuple_windows()
                    .all(|(&top, &bottom)| calc_white_num(top, bottom, left, mid) <= k)
                {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            if ok == left {
                return None;
            }

            if ok == w {
                break;
            }

            div_cnt += 1;

            left = ok;
        }

        Some(div_cnt)
    };

    let ans = (0_usize..1 << (h - 1))
        .filter_map(|row_div_bits| {
            let bounds = chain!(
                [0],
                (1..h).filter(|&row| row_div_bits >> (row - 1) & 1 == 1),
                [h]
            )
            .collect_vec();

            let hor_div_num = row_div_bits.count_ones();
            let ver_div_num = count_ver_div(&bounds)?;

            Some(hor_div_num + ver_div_num)
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
