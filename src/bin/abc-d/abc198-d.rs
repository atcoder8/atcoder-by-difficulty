use itertools::{join, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    if let Some(nn) = solve() {
        println!("{}", join(nn, "\n"));
    } else {
        println!("UNSOLVABLE");
    }
}

fn solve() -> Option<Vec<usize>> {
    input! {
        ss: [Chars; 3],
    }

    let mut tt = ss.iter().map(|s| vec![0; s.len()]).collect_vec();
    let mut char_type_cnt = 0;
    let mut trans = vec![None; 26];
    for (i, s) in ss.iter().enumerate() {
        for (j, &c) in s.iter().enumerate() {
            let c_idx = (c as u8 - b'a') as usize;

            if trans[c_idx].is_none() {
                trans[c_idx] = Some(char_type_cnt);
                char_type_cnt += 1;
            }

            tt[i][j] = trans[c_idx].unwrap();
        }
    }

    for perm in (0_usize..10).permutations(char_type_cnt) {
        if tt.iter().any(|t| perm[t[0]] == 0) {
            continue;
        }

        let nn = tt
            .iter()
            .map(|t| t.iter().fold(0, |acc, &t| 10 * acc + perm[t]))
            .collect_vec();

        if nn[0] + nn[1] == nn[2] {
            return Some(nn);
        }
    }

    None
}
