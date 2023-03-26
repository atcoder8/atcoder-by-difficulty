use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut heap = BinaryHeap::new();
    let mut inc = 0;

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        match query_type {
            1 => {
                input! {
                    x: i64,
                }

                heap.push(Reverse(x - inc));
            }
            2 => {
                input! {
                    x: i64,
                }

                inc += x;
            }
            _ => {
                let ans = heap.pop().unwrap().0 + inc;
                println!("{}", ans);
            }
        }
    }
}
