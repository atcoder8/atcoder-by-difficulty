use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    }

    let c_to_i = |c: u8| (c - b'A') as usize;
    let next_c = |c: u8| b'A' + (c - b'A' + 1) % 3;

    let mut counts = [0; 3];
    for &c in &s[..n] {
        counts[(c - b'A') as usize] += 1;
    }

    let mut labels = vec![0; 3 * n];

    for c1 in b'A'..=b'C' {
        let label1 = 2 * c_to_i(c1) + 1;
        let label2 = label1 + 1;
        let c2 = next_c(c1);
        let c3 = next_c(c2);
        let mut idx1 = 0;
        let mut idx2 = n;
        let mut c2_cnt = 0;
        let mut c3_cnt = 0;

        loop {
            while idx1 < n && s[idx1] != c1 {
                idx1 += 1;
            }

            if idx1 == n {
                break;
            }

            while s[idx2] == c1 || labels[idx2] != 0 {
                idx2 += 1;
            }

            if s[idx2] == c2 {
                labels[idx1] = label1;
                labels[idx2] = label1;
                c2_cnt += 1;
            } else {
                labels[idx1] = label2;
                labels[idx2] = label2;
                c3_cnt += 1;
            }

            idx1 += 1;
            idx2 += 1;
        }

        for i in idx2..3 * n {
            if labels[i] != 0 {
                continue;
            }

            if s[i] == c3 && c2_cnt > 0 {
                labels[i] = label1;
                c2_cnt -= 1;
            }

            if s[i] == c2 && c3_cnt > 0 {
                labels[i] = label2;
                c3_cnt -= 1;
            }
        }
    }

    println!("{}", labels.iter().join(""));
}
