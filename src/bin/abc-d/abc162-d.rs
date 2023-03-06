use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let s: Vec<usize> = s
        .iter()
        .map(|&c| match c {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => panic!(),
        })
        .collect_vec();

    let mut rgb = [0_usize; 3];
    for &x in &s {
        rgb[x] += 1;
    }

    let mut ans = 0;

    for i in 0..n {
        let color1 = s[i];

        rgb[color1] -= 1;

        let mut rem_rgb = rgb.clone();

        for j in (i + 1)..n {
            let color2 = s[j];

            rem_rgb[color2] -= 1;

            if color1 == color2 {
                continue;
            }

            let other_color = 3 - (s[i] + s[j]);

            ans += rem_rgb[other_color];

            let k = 2 * j - i;
            if k < n && s[k] == other_color {
                ans -= 1;
            }
        }
    }

    println!("{}", ans);
}
