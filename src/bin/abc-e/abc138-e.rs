use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        s: String,
        t: Chars,
    }

    let mut positions_each_c = vec![vec![]; 26];
    for (i, c) in s.chars().enumerate() {
        positions_each_c[char_to_int(c)].push(i);
    }

    if t.iter()
        .any(|&c| positions_each_c[char_to_int(c)].is_empty())
    {
        return None;
    }

    let calc_dist = |left: usize, c: char| {
        let char_id = char_to_int(c);
        let positions = &positions_each_c[char_id];
        let cnt = positions.lower_bound(&left);

        if cnt < positions.len() {
            positions[cnt] - left + 1
        } else {
            s.len() - left + positions[0] + 1
        }
    };

    let mut ans = 0;
    let mut left = 0;
    for &c in &t {
        let dist = calc_dist(left, c);
        ans += dist;
        left = (left + dist) % s.len();
    }

    Some(ans)
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
