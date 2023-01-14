use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        c: char,
    }

    let vowel = "aeiou".chars().collect_vec();

    println!(
        "{}",
        if vowel.contains(&c) {
            "vowel"
        } else {
            "consonant"
        }
    );
}
