use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut front = vec![None; n];
    let mut back = vec![None; n];

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        match query_type {
            1 => {
                input! {
                    (x, y): (Usize1, Usize1),
                }

                back[x] = Some(y);
                front[y] = Some(x);
            }
            2 => {
                input! {
                    (x, y): (Usize1, Usize1),
                }

                back[x] = None;
                front[y] = None;
            }
            3 => {
                input! {
                    x: Usize1,
                }

                let mut forward = vec![];
                let mut idx = front[x];
                while let Some(idx_) = idx {
                    forward.push(idx_);
                    idx = front[idx_];
                }
                forward.reverse();

                let mut backward = vec![];
                let mut idx = back[x];
                while let Some(idx_) = idx {
                    backward.push(idx_);
                    idx = back[idx_];
                }

                let component = forward
                    .into_iter()
                    .chain(vec![x])
                    .chain(backward)
                    .collect_vec();
                println!(
                    "{} {}",
                    component.len(),
                    component.iter().map(|x| (x + 1).to_string()).join(" ")
                );
            }
            _ => panic!(),
        }
    }
}
