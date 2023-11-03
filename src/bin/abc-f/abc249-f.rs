use std::{cmp::Reverse, collections::BinaryHeap};

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
    let mut rem_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut del_replace_cnt = 0;
    for (i, &(t, y)) in enumerate(&ty).rev() {
        if t == 1 {
            ans = ans.max(y + sum);

            if del_replace_cnt == k {
                break;
            }

            if del_heap.len() + del_replace_cnt == k {
                let add_val = del_heap.pop().unwrap();
                rem_heap.push(Reverse(add_val));
                sum += add_val;
            }
            del_replace_cnt += 1;

            ans = ans.max(xx[i] + sum);
        } else {
            if y >= 0 {
                sum += y;
            } else if del_heap.len() + del_replace_cnt < k {
                del_heap.push(y);
            } else {
                rem_heap.push(Reverse(y));
                sum += y;

                let del_val = rem_heap.pop().unwrap().0;
                del_heap.push(del_val);
                sum -= del_val;

                let add_val = del_heap.pop().unwrap();
                rem_heap.push(Reverse(add_val));
                sum += add_val;
            }

            ans = ans.max(xx[i] + sum);
        }
    }

    println!("{}", ans);
}
