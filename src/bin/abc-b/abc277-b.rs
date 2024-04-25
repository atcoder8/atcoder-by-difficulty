use itertools::Itertools;
use proconio::input;
use regex::Regex;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let re = Regex::new(r"^[HDCS][A2-9TJQK]").unwrap();

    let ans = ss.iter().all_unique() && ss.iter().all(|s| re.is_match(&s));
    println!("{}", if ans { "Yes" } else { "No" });
}
