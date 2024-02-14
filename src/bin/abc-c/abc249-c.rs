use fixedbitset::FixedBitSet;
use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        ss: [String; n],
    }

    let create_contains_set = |s: &str| {
        let mut contains_set = FixedBitSet::with_capacity(26);
        for c in s.chars() {
            contains_set.insert(char_to_number(c));
        }

        contains_set
    };

    let contains_sets = ss.iter().map(|s| create_contains_set(s)).collect_vec();

    let count_match_types = |bit: usize| {
        let mut counts = vec![0; 26];
        for (_, contains_set) in
            enumerate(&contains_sets).filter(|(s_idx, _)| bit >> *s_idx & 1 == 1)
        {
            for i in 0..26 {
                counts[i] += contains_set[i] as usize;
            }
        }

        counts.iter().filter(|&&cnt| cnt == k).count()
    };

    let ans = (0..1 << n).map(count_match_types).max().unwrap();
    println!("{}", ans);
}

/// Converts the character `c` to the corresponding numeric value.
pub fn char_to_number(c: char) -> usize {
    (c as u8 - b'a') as usize
}
