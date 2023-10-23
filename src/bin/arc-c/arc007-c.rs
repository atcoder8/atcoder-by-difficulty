use proconio::{input, marker::Chars};

fn main() {
    input! {
        cc: Chars,
    }

    let n = cc.len() as u32;

    let mask = (1 << n) - 1;

    let pattern: u32 = cc
        .iter()
        .enumerate()
        .map(|(i, &c)| ((c == 'o') as u32) << i)
        .sum();

    let rotate = |pattern: u32, dist: u32| (pattern << dist & mask) | (pattern >> n - dist);

    let is_ok = |bit: u32| {
        (0..n)
            .filter(|&dist| bit >> dist & 1 == 1)
            .map(|dist| rotate(pattern, dist))
            .fold(0, |acc, rotated_pattern| acc | rotated_pattern)
            == mask
    };

    let ans = (0..1 << n)
        .filter(|&bit| is_ok(bit))
        .map(|bit| bit.count_ones())
        .min()
        .unwrap();
    println!("{}", ans);
}
