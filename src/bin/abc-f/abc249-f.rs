use std::collections::BinaryHeap;

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        ty: [(usize, i64); n],
    }

    let mut xx = vec![0];
    for &(t, y) in &ty {
        if t == 1 {
            xx.push(y);
        } else {
            xx.push(*xx.last().unwrap() + y);
        }
    }

    let mut ans = *xx.last().unwrap();
    let mut sum = 0;
    let mut del_heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut assign_cnt = 0;
    for (i, &(t, y)) in enumerate(&ty).rev() {
        if t == 1 {
            if assign_cnt == k {
                break;
            }

            assign_cnt += 1;

            if del_heap.len() + assign_cnt > k {
                sum += del_heap.pop().unwrap();
            }

            ans = ans.max(xx[i] + sum);
        } else {
            if y >= 0 {
                sum += y;
            } else {
                del_heap.push(y);
                if del_heap.len() + assign_cnt > k {
                    sum += del_heap.pop().unwrap();
                }
            }

            ans = ans.max(xx[i] + sum);
        }
    }

    println!("{}", ans);
}
