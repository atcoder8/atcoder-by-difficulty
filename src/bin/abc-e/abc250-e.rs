use im_rc::HashSet;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
        q: usize,
        xy: [(usize, usize); q],
    }

    let mut boundary = vec![];
    let mut a_to_b = vec![];
    let mut a_set = HashSet::new();
    let mut b_set = HashSet::new();
    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut diff_cnt = 0;
    loop {
        while a_idx < n && a_set.contains(&aa[a_idx]) {
            a_idx += 1;
        }

        if a_idx == n {
            break;
        }

        let a = aa[a_idx];

        boundary.push(a_idx);
        a_set.insert(a);

        if b_set.contains(&a) {
            diff_cnt -= 1;
        } else {
            diff_cnt += 1;
        }

        if b_idx < n {
            b_set.insert(bb[b_idx]);
            if a_set.contains(&bb[b_idx]) {
                diff_cnt -= 1;
            } else {
                diff_cnt += 1;
            }
        }

        let mut match_set = HashSet::new();
        while b_idx < n && b_set.contains(&bb[b_idx]) {
            if diff_cnt == 0 {
                match_set.insert(b_idx);
            }

            b_idx += 1;
        }
        a_to_b.push(match_set);
    }

    for &(x, y) in &xy {
        let pos = boundary.upper_bound(&(x - 1)) - 1;
        let ans = a_to_b[pos].contains(&(y - 1));
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
