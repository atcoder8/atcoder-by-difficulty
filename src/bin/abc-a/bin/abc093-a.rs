use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    s.sort_unstable();

    println!(
        "{}",
        if s == "abc".chars().collect::<Vec<char>>() {
            "Yes"
        } else {
            "No"
        }
    );
}
