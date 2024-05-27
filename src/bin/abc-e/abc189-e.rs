use itertools::Itertools;
use ndarray::{array, Array2};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        m: usize,
    }

    let mut prefix_prod = vec![Array2::<i64>::eye(3)];
    prefix_prod.reserve(m);

    for i in 0..m {
        input! {
            op_type: usize,
        }

        let mat = match op_type {
            1 => array![[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
            2 => array![[0, -1, 0], [1, 0, 0], [0, 0, 1]],
            3 => {
                input! {
                    p: i64,
                }

                array![[-1, 0, 2 * p], [0, 1, 0], [0, 0, 1]]
            }
            4 => {
                input! {
                    p: i64,
                }

                array![[1, 0, 0], [0, -1, 2 * p], [0, 0, 1]]
            }
            _ => unreachable!(),
        };

        let prod = mat.dot(&prefix_prod[i]);
        prefix_prod.push(prod);
    }

    let solve = |a: usize, b: usize| {
        let (x, y) = xy[b];
        prefix_prod[a].dot(&array![x, y, 1])
    };

    input! {
        q: usize,
        ab: [(usize, Usize1); q],
    }

    let ans = ab
        .iter()
        .map(|&(a, b)| {
            let moved_vec = solve(a, b);
            format!("{} {}", moved_vec[0], moved_vec[1])
        })
        .join("\n");
    println!("{}", ans);
}
