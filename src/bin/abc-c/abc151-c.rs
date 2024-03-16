use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ps: [(Usize1, String); m],
    }

    let mut accepted = vec![false; n];
    let mut penalty = vec![0; n];
    for (p, s) in ps {
        if accepted[p] {
            continue;
        }

        if s == "AC" {
            accepted[p] = true;
        } else {
            penalty[p] += 1;
        }
    }

    let mut ac_num = 0;
    let mut penalty_num = 0;
    for i in 0..n {
        if accepted[i] {
            ac_num += 1;
            penalty_num += penalty[i];
        }
    }

    println!("{} {}", ac_num, penalty_num);
}
